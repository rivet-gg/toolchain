use cli_core::rivet_api::{apis, models};
use futures_util::stream::{StreamExt, TryStreamExt};
use global_error::prelude::*;
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

pub async fn push_tar(ctx: &cli_core::Ctx, push_opts: &PushOpts) -> GlobalResult<PushOutput> {
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

	let build_res = apis::cloud_games_builds_api::cloud_games_builds_create_game_build(
		&ctx.openapi_config_cloud,
		&ctx.game_id,
		models::CloudGamesCreateGameBuildRequest {
			display_name: display_name.clone(),
			image_tag: push_opts.tag.clone(),
			image_file: Box::new(models::UploadPrepareFile {
				path: "image.tar".into(),
				content_type: Some(content_type.into()),
				content_length: image_file_meta.len() as i64,
			}),
			kind: Some(match push_opts.kind {
				BuildKind::DockerImage => models::CloudGamesBuildKind::DockerImage,
				BuildKind::OciBundle => models::CloudGamesBuildKind::OciBundle,
			}),
			compression: Some(match push_opts.compression {
				BuildCompression::None => models::CloudGamesBuildCompression::None,
				BuildCompression::Lz4 => models::CloudGamesBuildCompression::Lz4,
			}),
			multipart_upload: Some(multipart_enabled()),
		},
	)
	.await;
	if let Err(err) = build_res.as_ref() {
		println!("Error: {err:?}");
	}
	let build_res = unwrap!(build_res,);
	let image_id = build_res.build_id;
	let mpb = indicatif::MultiProgress::new();

	if multipart_enabled() {
		// Upload chunks in parallel
		futures_util::stream::iter(build_res.image_presigned_requests.unwrap())
			.map(|presigned_request| {
				let reqwest_client = reqwest_client.clone();
				let mpb = mpb.clone();

				async move {
					upload::upload_file(
						&reqwest_client,
						&presigned_request,
						&push_opts.path,
						Some(content_type),
						mpb,
					)
					.await
				}
			})
			.buffer_unordered(8)
			.try_collect::<Vec<_>>()
			.await?;
	} else {
		// Upload file
		upload::upload_file(
			&reqwest_client,
			&build_res.image_presigned_request.unwrap(),
			&push_opts.path,
			Some(content_type),
			mpb.clone(),
		)
		.await?;
	}

	eprintln!("\n");
	let complete_res = apis::cloud_uploads_api::cloud_uploads_complete_upload(
		&ctx.openapi_config_cloud,
		&build_res.upload_id.to_string(),
	)
	.await;
	if let Err(err) = complete_res.as_ref() {
		println!("Error: {err:?}");
	}
	unwrap!(complete_res);
	term::status::success("Image Upload Complete", image_id);

	Ok(PushOutput {
		image_id: image_id.to_owned(),
	})
}

fn multipart_enabled() -> bool {
	!std::env::var("_RIVET_UPLOAD_DISABLE_MULTIPART")
		.ok()
		.map_or(false, |x| &x == "1")
}
