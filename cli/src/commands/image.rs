use anyhow::{bail, Context, Result};
use clap::Parser;
use cli_core::rivet_api;
use rand::{thread_rng, Rng};
use serde::Serialize;
use std::sync::Arc;
use tokio::fs;

use crate::util::{struct_fmt, upload};

#[derive(Parser)]
pub enum SubCommand {
	/// Pushes a image to Rivet so it can be used in a version
	Push(ImagePushOpts),
}

#[derive(Parser)]
pub struct ImagePushOpts {
	/// Docker tag to push
	#[clap(long)]
	pub tag: String,

	/// Name of the image
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
	pub image_id: String,
}

pub async fn push(ctx: &cli_core::Ctx, push_opts: &ImagePushOpts) -> Result<PushOutput> {
	let reqwest_client = Arc::new(reqwest::Client::new());

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

	// Create imag
	let display_name = push_opts
		.name
		.clone()
		.unwrap_or_else(|| push_opts.tag.clone());
	let content_type = "application/x-tar";
	eprintln!(
		"\n\n> Pushing image \"{name}\" ({size})",
		name = display_name,
		size = upload::format_file_size(image_file_meta.len())?,
	);
	let build_res = rivet_api::apis::cloud_games_builds_api::cloud_games_builds_create_game_build(
		&ctx.openapi_config_cloud,
		&ctx.game_id,
		rivet_api::models::CloudGamesCreateGameBuildInput {
			display_name: display_name.clone(),
			image_tag: image_tag.clone(),
			image_file: Box::new(rivet_api::models::UploadPrepareFile {
				path: "image.tar".into(),
				content_type: Some(content_type.into()),
				content_length: image_file_meta.len() as i64,
			}),
		},
	)
	.await;
	if let Err(err) = build_res.as_ref() {
		println!("Error: {err:?}");
	}
	let build_res = build_res.context("cloud_games_builds_create_game_build")?;
	let image_id = build_res.build_id;

	eprintln!(
		"\n\n> Uploading ({size})",
		size = upload::format_file_size(image_file_meta.len())?,
	);
	upload::upload_file(
		&reqwest_client,
		&build_res.image_presigned_request,
		tmp_path,
		Some(content_type),
	)
	.await?;

	eprintln!("\n\n> Completing");
	let complete_res = rivet_api::apis::cloud_uploads_api::cloud_uploads_complete_upload(
		&ctx.openapi_config_cloud,
		&build_res.upload_id,
		serde_json::json!({}),
	)
	.await;
	if let Err(err) = complete_res.as_ref() {
		println!("Error: {err:?}");
	}
	complete_res.context("cloud_uploads_complete_upload")?;

	Ok(PushOutput {
		image_id: image_id.to_owned(),
	})
}
