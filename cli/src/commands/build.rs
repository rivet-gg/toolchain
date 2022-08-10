use std::sync::Arc;

use anyhow::{bail, Context, Result};
use clap::Parser;
use rand::{thread_rng, Rng};
use tokio::fs;

use crate::util::{game, upload};

#[derive(Parser)]
pub enum SubCommand {
	Push(BuildPushOpts),
}

#[derive(Parser)]
pub struct BuildPushOpts {
	#[clap(index(1))]
	pub tag: String,

	#[clap(long)]
	pub name: Option<String>,
}

impl SubCommand {
	pub async fn execute(&self, ctx: &rivetctl::Ctx) -> Result<()> {
		match self {
			SubCommand::Push(push_opts) => {
				let reqwest_client = Arc::new(reqwest::Client::new());

				let game_id = game::infer_game_id(&ctx).await?;

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
					size = upload::format_file_size(image_file_meta.len())?,
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
					size = upload::format_file_size(image_file_meta.len())?,
				);
				upload::upload_file(
					&reqwest_client,
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

				Ok(())
			}
		}
	}
}
