use std::sync::Arc;

use anyhow::{bail, Context, Result};
use clap::Parser;
use rand::{thread_rng, Rng};
use serde::Serialize;
use tokio::fs;

use crate::util::{game, struct_fmt, upload};

#[derive(Parser)]
pub enum SubCommand {
	Push(BuildPushOpts),
}

#[derive(Parser)]
pub struct BuildPushOpts {
	#[clap(long)]
	pub tag: String,

	#[clap(long)]
	pub name: Option<String>,

	#[clap(long, value_parser)]
	pub format: Option<struct_fmt::Format>,
}

impl SubCommand {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> Result<()> {
		match self {
			SubCommand::Push(push_opts) => {
				let output = push(ctx, push_opts).await?;
				struct_fmt::print_opt(push_opts.format.as_ref(), &output)?;
				Ok(())
			}
		}
	}
}

#[derive(Serialize)]
pub struct PushOutput {
	pub build_id: String,
}

pub async fn push(ctx: &cli_core::Ctx, push_opts: &BuildPushOpts) -> Result<PushOutput> {
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
	eprintln!("\n\n> Archiving image");
	let tag_cmd = tokio::process::Command::new("docker")
		.arg("image")
		.arg("tag")
		.arg(&push_opts.tag)
		.arg(&image_tag)
		.output()
		.await?;
	if !tag_cmd.status.success() {
		eprintln!(
			"  ! Failed to archive Docker image:\n\nStatus: {}\n\nStdout:\n{}\n\nStderr:\n{}",
			tag_cmd.status,
			String::from_utf8_lossy(&tag_cmd.stdout),
			String::from_utf8_lossy(&tag_cmd.stderr)
		);
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
		eprintln!(
			"  ! Failed to archive Docker image:\n\nStatus: {}\n\nStdout:\n{}\n\nStderr:\n{}",
			save_cmd.status,
			String::from_utf8_lossy(&save_cmd.stdout),
			String::from_utf8_lossy(&save_cmd.stderr)
		);
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
	eprintln!(
		"\n\n> Creating build \"{name}\" ({size})",
		name = display_name,
		size = upload::format_file_size(image_file_meta.len())?,
	);
	let build_res = ctx
		.client()
		.create_game_build()
		.game_id(&game_id)
		.display_name(&display_name)
		.image_tag(&image_tag)
		.image_file(
			cli_core::rivet_cloud::model::upload_prepare_file::Builder::default()
				.path("image.tar")
				.content_type(content_type)
				.content_length(image_file_meta.len() as i64)
				.build(),
		)
		.send()
		.await
		.context("client.create_game_build")?;
	let build_id = build_res.build_id().context("build_res.build_id")?;

	eprintln!(
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

	eprintln!("\n\n> Completing");
	ctx.client()
		.complete_upload()
		.upload_id(build_res.upload_id().unwrap())
		.send()
		.await
		.context("client.complete_upload")?;

	Ok(PushOutput {
		build_id: build_id.to_owned(),
	})
}
