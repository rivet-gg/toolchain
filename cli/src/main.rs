use anyhow::{bail, Context, Result};
use clap::Parser;
use commands::*;
use futures_util::stream::{StreamExt, TryStreamExt};
use rand::{thread_rng, Rng};
use std::{
	env,
	path::{Path, PathBuf},
	sync::{
		atomic::{AtomicU64, AtomicUsize, Ordering},
		Arc,
	},
	time::{Duration, Instant},
};
use tokio::fs;

mod commands;
mod util;

#[derive(Parser)]
#[clap()]
struct Opts {
	#[clap(subcommand)]
	command: SubCommand,

	#[clap(long, env = "RIVET_CLOUD_API_URL")]
	api_url: Option<String>,

	#[clap(long, env = "RIVET_CLOUD_ACCESS_TOKEN")]
	access_token: Option<String>,
}

#[derive(Parser)]
enum SubCommand {
	Auth {
		#[clap(subcommand)]
		command: auth::SubCommand,
	},
	Build {
		#[clap(subcommand)]
		command: build::SubCommand,
	},
	Site {
		#[clap(subcommand)]
		command: site::SubCommand,
	},
}

#[tokio::main]
async fn main() -> Result<()> {
	let opts = Opts::parse();

	// Read config
	let config_path = home::home_dir()
		.context("missing home dir")?
		.join(".config")
		.join("rivetctl.json");
	let config = match fs::read(&config_path).await {
		Result::Ok(buf) => serde_json::from_slice::<rivetctl::config::Config>(buf.as_slice())?,
		Result::Err(_) => rivetctl::config::Config::default(),
	};

	// Build ctx
	let ctx = rivetctl::ctx::SharedCtx::new(
		config.clone(),
		opts.api_url.clone(),
		opts.access_token.clone(),
	)
	.await?;

	match opts.command {
		SubCommand::Auth { command } => match command {
			auth::SubCommand::Token { .. } => {
				print!("Auth token: ");

				// Read token from stdin
				let token = tokio::task::spawn_blocking(|| {
					use std::io::BufRead;

					let stdin = std::io::stdin();
					let mut iterator = stdin.lock().lines();
					iterator.next().unwrap().context("token not provided")
				})
				.await??;

				// Create new config
				let mut new_config = config.clone();
				new_config.auth.token = Some(token.trim().to_owned());

				// Create new context without overridden access token to check the token
				let new_ctx =
					rivetctl::ctx::SharedCtx::new(new_config.clone(), opts.api_url.clone(), None)
						.await?;
				let inspect = new_ctx
					.http_client
					.inspect()
					.send()
					.await
					.context("http_client.inspect")?;
				println!("{:?}", inspect);

				// Save new config
				write_config(&new_config, &config_path).await?;
			}
		},
		SubCommand::Build { command } => match command {
			build::SubCommand::Push(push_opts) => {
				let game_id = infer_game_id(&ctx).await?;

				let tmp_image_file = tempfile::NamedTempFile::new()?;
				let tmp_path = tmp_image_file.into_temp_path();

				// Re-tag and archive the image
				let image_tag_tag = thread_rng()
					.sample_iter(rand::distributions::Alphanumeric)
					.map(char::from)
					.take(16)
					.collect::<String>()
					.to_lowercase();
				let image_tag = format!("rivet-game:{}", image_tag_tag);
				println!("\n\n> Archiving image");
				let tag_cmd = tokio::process::Command::new("docker")
					.arg("image")
					.arg("tag")
					.arg(&push_opts.tag)
					.arg(&image_tag)
					.output()
					.await?;
				if !tag_cmd.status.success() {
					eprintln!("  ! Failed to archive Docker image:\n\nStatus: {}\n\nStdout:\n{}\n\nStderr:\n{}", tag_cmd.status, String::from_utf8_lossy(&tag_cmd.stdout), String::from_utf8_lossy(&tag_cmd.stderr));
					bail!("failed to tag docker image");
				}

				let save_cmd = tokio::process::Command::new("docker")
					.arg("image")
					.arg("save")
					.arg("--output")
					.arg(&tmp_path)
					.arg(&image_tag)
					.output()
					.await?;
				if !save_cmd.status.success() {
					eprintln!("  ! Failed to archive Docker image:\n\nStatus: {}\n\nStdout:\n{}\n\nStderr:\n{}", save_cmd.status, String::from_utf8_lossy(&save_cmd.stdout), String::from_utf8_lossy(&save_cmd.stderr));
					bail!("failed to save docker image");
				}

				// Inspect the image
				let image_file_meta = fs::metadata(&tmp_path).await?;

				// Create build
				let display_name = push_opts
					.name
					.clone()
					.unwrap_or_else(|| push_opts.tag.clone());
				let content_type = "application/x-tar";
				println!(
					"\n\n> Creating build \"{name}\" ({size})",
					name = display_name,
					size = format_file_size(image_file_meta.len())?,
				);
				let build_res = ctx
					.http_client
					.create_game_build()
					.game_id(&game_id)
					.display_name(&display_name)
					.image_tag(&image_tag)
					.image_file(
						rivetctl::model::upload_prepare_file::Builder::default()
							.path("image.tar")
							.content_type(content_type)
							.content_length(image_file_meta.len() as i64)
							.build(),
					)
					.send()
					.await
					.context("http_client.create_game_build")?;

				println!(
					"\n\n> Uploading ({size})",
					size = format_file_size(image_file_meta.len())?,
				);
				upload_file(
					&client,
					&build_res.image_presigned_request().unwrap(),
					tmp_path,
					Some(content_type),
				)
				.await?;

				println!("\n\n> Completing");
				ctx.http_client
					.complete_upload()
					.upload_id(build_res.upload_id().unwrap())
					.send()
					.await
					.context("http_client.complete_upload")?;
			}
		},
		SubCommand::Site { command } => command.execute(&ctx).await?,
	}

	Ok(())
}

/// Writes a modified config to the file system.
async fn write_config(config: &rivetctl::config::Config, path: &Path) -> Result<()> {
	// Create parent directory
	fs::create_dir_all(&path.parent().context("config path parent")?).await?;

	// Write config
	let config_str = serde_json::to_string(config)?;
	fs::write(path, config_str).await?;

	Ok(())
}
