use anyhow::*;
use futures_util::stream::{StreamExt, TryStreamExt};
use rivet_api::{apis, models};
use serde::Serialize;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::fs;
use uuid::Uuid;

use crate::{
	config,
	toolchain_ctx::ToolchainCtx,
	util::{net::upload, task, term},
};

use super::{BuildCompression, BuildKind};

pub struct PushOpts {
	pub env_id: Uuid,

	/// Path to already created tar.
	pub path: PathBuf,

	pub tags: HashMap<String, String>,
	pub exclusive_tags: Vec<String>,

	/// Docker inside the image.
	pub docker_tag: String,

	/// Name of the image
	pub name: Option<String>,

	pub kind: BuildKind,

	pub compression: BuildCompression,
}

#[derive(Serialize)]
pub struct PushOutput {
	pub image_id: Uuid,
}

pub async fn push_tar(
	ctx: &ToolchainCtx,
	task: task::TaskCtx,
	push_opts: &PushOpts,
) -> Result<PushOutput> {
	let multipart_enabled =
		config::settings::try_read(|x| Ok(!x.net.disable_upload_multipart)).await?;

	let reqwest_client = Arc::new(reqwest::Client::new());

	let game_id_str = ctx.game_id.to_string();
	let env_id_str = push_opts.env_id.to_string();

	// Inspect the image
	let image_file_meta = fs::metadata(&push_opts.path).await?;
	ensure!(image_file_meta.len() > 0, "docker image archive is empty");

	// Create image
	let display_name = push_opts
		.name
		.clone()
		.unwrap_or_else(|| push_opts.docker_tag.clone());
	let content_type = "binary/octet-stream";

	task.log(format!(
		"[Uploading Image] {name} ({size})",
		name = display_name,
		size = upload::format_file_size(image_file_meta.len())?
	));

	let build_res = apis::servers_builds_api::servers_builds_prepare_build(
		&ctx.openapi_config_cloud,
		&game_id_str,
		&env_id_str,
		models::ServersCreateBuildRequest {
			name: display_name.clone(),
			image_tag: push_opts.docker_tag.clone(),
			image_file: Box::new(models::UploadPrepareFile {
				path: "image.tar".into(),
				content_type: Some(content_type.into()),
				content_length: image_file_meta.len() as i64,
			}),
			kind: Some(match push_opts.kind {
				BuildKind::DockerImage => models::ServersBuildKind::DockerImage,
				BuildKind::OciBundle => models::ServersBuildKind::OciBundle,
			}),
			compression: Some(match push_opts.compression {
				BuildCompression::None => models::ServersBuildCompression::None,
				BuildCompression::Lz4 => models::ServersBuildCompression::Lz4,
			}),
			multipart_upload: Some(multipart_enabled),
		},
	)
	.await;
	if let Err(err) = build_res.as_ref() {
		task.log(format!("{err:?}"))
	}
	let build_res = build_res.context("build_res")?;
	let image_id = build_res.build;
	let pb = term::EitherProgressBar::Multi(term::multi_progress_bar(task.clone()));

	if multipart_enabled {
		// Upload chunks in parallel
		futures_util::stream::iter(build_res.image_presigned_requests.unwrap())
			.map(|presigned_request| {
				let task = task.clone();
				let reqwest_client = reqwest_client.clone();
				let pb = pb.clone();

				async move {
					upload::upload_file(
						task.clone(),
						&reqwest_client,
						&presigned_request,
						&push_opts.path,
						Some(content_type),
						pb,
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
			task.clone(),
			&reqwest_client,
			&build_res.image_presigned_request.unwrap(),
			&push_opts.path,
			Some(content_type),
			pb,
		)
		.await?;
	}

	let complete_res = apis::servers_builds_api::servers_builds_complete_build(
		&ctx.openapi_config_cloud,
		&game_id_str,
		&env_id_str,
		&build_res.build.to_string(),
	)
	.await;
	if let Err(err) = complete_res.as_ref() {
		task.log(format!("{err:?}"));
	}
	complete_res.context("complete_res")?;

	let complete_res = apis::servers_builds_api::servers_builds_patch_tags(
		&ctx.openapi_config_cloud,
		&game_id_str,
		&env_id_str,
		&build_res.build.to_string(),
		models::ServersPatchBuildTagsRequest {
			tags: Some(serde_json::to_value(&push_opts.tags)?),
			exclusive_tags: Some(push_opts.exclusive_tags.clone()),
		},
	)
	.await;
	if let Err(err) = complete_res.as_ref() {
		task.log(format!("{err:?}"));
	}
	complete_res.context("complete_res")?;

	task.log(format!("[Image Upload Complete] {image_id}"));

	Ok(PushOutput {
		image_id: image_id.to_owned(),
	})
}
