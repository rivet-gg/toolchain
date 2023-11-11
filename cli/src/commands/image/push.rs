use anyhow::{ensure, Context, Result};
use cli_core::rivet_api;
use serde::Serialize;
use std::{path::PathBuf, sync::Arc};
use tokio::fs;
use uuid::Uuid;

use crate::util::{struct_fmt, term, upload};

use super::{BuildCompression, BuildKind};

pub struct PushOpts {
	/// Path to already created tar.
	pub path: PathBuf,

	/// Docker inside the image.
	pub tag: String,

	/// Name of the image
	pub name: Option<String>,

	pub kind: BuildKind,

	pub compression: BuildCompression,

	pub format: Option<struct_fmt::Format>,
}

#[derive(Serialize)]
pub struct PushOutput {
	pub image_id: Uuid,
}

pub async fn push_tar(ctx: &cli_core::Ctx, push_opts: &PushOpts) -> Result<PushOutput> {
	let reqwest_client = Arc::new(reqwest::Client::new());

	// Inspect the image
	let image_file_meta = fs::metadata(&push_opts.path).await?;
	ensure!(image_file_meta.len() > 0, "docker image archive is empty");

	// Create image
	let display_name = push_opts
		.name
		.clone()
		.unwrap_or_else(|| push_opts.tag.clone());
	let content_type = "binary/octet-stream";

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
				content_type: Some(content_type.into()),
				content_length: image_file_meta.len() as i64,
			}),
			kind: Some(match push_opts.kind {
				BuildKind::DockerImage => rivet_api::models::CloudGamesBuildKind::DockerImage,
				BuildKind::OciBundle => rivet_api::models::CloudGamesBuildKind::OciBundle,
			}),
			compression: Some(match push_opts.compression {
				BuildCompression::None => rivet_api::models::CloudGamesBuildCompression::None,
				BuildCompression::Lz4 => rivet_api::models::CloudGamesBuildCompression::Lz4,
			}),
			multipart_upload: Some(multipart_enabled()),
		},
	)
	.await;
	if let Err(err) = build_res.as_ref() {
		println!("Error: {err:?}");
	}
	let build_res = build_res.context("cloud_games_builds_create_game_build")?;
	let image_id = build_res.build_id;

	if multipart_enabled() {
		for presigned_request in build_res.image_presigned_requests.unwrap() {
			upload::upload_file(
				&reqwest_client,
				&presigned_request,
				&push_opts.path,
				Some(content_type),
			)
			.await?;
		}
	} else {
		upload::upload_file(
			&reqwest_client,
			&build_res.image_presigned_request.unwrap(),
			&push_opts.path,
			Some(content_type),
		)
		.await?;
	}

	println!("cloud config: {:?}", ctx.openapi_config_cloud);
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

fn multipart_enabled() -> bool {
	!std::env::var("_RIVET_UPLOAD_DISABLE_MULTIPART")
		.ok()
		.map_or(false, |x| &x == "1")
}
