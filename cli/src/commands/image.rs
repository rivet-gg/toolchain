use anyhow::{ensure, Context, Result};
use clap::Parser;
use cli_core::rivet_api;
use rand::{thread_rng, Rng};
use serde::Serialize;
use std::{path::PathBuf, sync::Arc};
use tokio::fs;
use uuid::Uuid;

use crate::util::{cmd, struct_fmt, term, upload};

#[derive(Parser)]
pub enum SubCommand {
	/// Pushes a image to Rivet so it can be used in a version
	Push(ImagePushTagOpts),
}

#[derive(Parser)]
pub struct ImagePushTagOpts {
	/// Docker tag to push
	#[clap(long)]
	pub tag: String,

	/// Name of the image
	#[clap(long)]
	pub name: Option<String>,

	#[clap(long, value_parser)]
	pub format: Option<struct_fmt::Format>,
}

#[derive(Parser)]
pub struct ImagePushTarOpts {
	/// Path to already created tar.
	#[clap(long)]
	pub path: PathBuf,

	/// Docker inside the image.
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
				let output = push_tag(ctx, push_opts).await?;
				struct_fmt::print_opt(push_opts.format.as_ref(), &output)?;
				Ok(())
			}
		}
	}
}

#[derive(Serialize)]
pub struct PushOutput {
	pub image_id: Uuid,
}

pub async fn push_tag(ctx: &cli_core::Ctx, push_opts: &ImagePushTagOpts) -> Result<PushOutput> {
	let tmp_image_file = tempfile::NamedTempFile::new()?;
	let tmp_path = tmp_image_file.into_temp_path();

	// Re-tag and archive the image
	eprintln!();
	term::status::info("Archiving Image", "");

	let image_tag_tag = thread_rng()
		.sample_iter(rand::distributions::Alphanumeric)
		.map(char::from)
		.take(16)
		.collect::<String>()
		.to_lowercase();
	let unique_image_tag = format!("rivet-game:{}", image_tag_tag);

	let mut tag_cmd = tokio::process::Command::new("docker");
	tag_cmd
		.arg("image")
		.arg("tag")
		.arg(&push_opts.tag)
		.arg(&unique_image_tag);
	cmd::execute_docker_cmd_silent(tag_cmd, "failed to tag Docker image").await?;

	let mut save_cmd = tokio::process::Command::new("docker");
	save_cmd
		.arg("image")
		.arg("save")
		.arg("--output")
		.arg(&tmp_path)
		.arg(&unique_image_tag);
	cmd::execute_docker_cmd_silent(save_cmd, "failed to archive Docker image").await?;

	push_tar(
		ctx,
		&ImagePushTarOpts {
			path: tmp_path.to_owned(),
			tag: unique_image_tag,
			name: push_opts.name.clone(),
			format: push_opts.format.clone(),
		},
	)
	.await
}

pub async fn push_tar(ctx: &cli_core::Ctx, push_opts: &ImagePushTarOpts) -> Result<PushOutput> {
	let reqwest_client = Arc::new(reqwest::Client::new());

	// Inspect the image
	let image_file_meta = fs::metadata(&push_opts.path).await?;
	ensure!(image_file_meta.len() > 0, "docker image archive is empty");

	// Create image
	let display_name = push_opts
		.name
		.clone()
		.unwrap_or_else(|| push_opts.tag.clone());
	eprintln!();
	term::status::info(
		"Uploading Image",
		format!(
			"{name} ({size})",
			name = display_name,
			size = upload::format_file_size(image_file_meta.len())?
		),
	);
	let build_res = rivet_api::apis::cloud_games_builds_api::cloud_games_builds_create_game_build(
		&ctx.openapi_config_cloud,
		&ctx.game_id,
		rivet_api::models::CloudGamesCreateGameBuildRequest {
			display_name: display_name.clone(),
			image_tag: push_opts.tag.clone(),
			image_file: Box::new(rivet_api::models::UploadPrepareFile {
				path: "image.tar".into(),
				content_type: None,
				content_length: image_file_meta.len() as i64,
			}),
			multipart_upload: Some(false),
		},
	)
	.await;
	if let Err(err) = build_res.as_ref() {
		println!("Error: {err:?}");
	}
	let build_res = build_res.context("cloud_games_builds_create_game_build")?;
	let image_id = build_res.build_id;

	upload::upload_file(
		&reqwest_client,
		build_res
			.image_presigned_request
			.as_ref()
			.context("image_presigned_request")?,
		&push_opts.path,
		None,
	)
	.await?;

	let complete_res = rivet_api::apis::cloud_uploads_api::cloud_uploads_complete_upload(
		&ctx.openapi_config_cloud,
		&build_res.upload_id.to_string(),
	)
	.await;
	if let Err(err) = complete_res.as_ref() {
		println!("Error: {err:?}");
	}
	complete_res.context("cloud_uploads_complete_upload")?;
	term::status::success("Image Upload Complete", "");

	Ok(PushOutput {
		image_id: image_id.to_owned(),
	})
}
