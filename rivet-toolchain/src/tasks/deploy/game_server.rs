use global_error::prelude::*;
use std::{collections::HashMap, path::Path};
use uuid::Uuid;

use crate::{
	ctx::Ctx,
	util::{
		cmd::{self, shell_cmd},
		docker::{self, generate_unique_image_tag, BuildCompression, BuildKind},
		task::TaskCtx,
	},
};

pub struct DeployOpts {
	pub display_name: String,
	pub build_dir: String,
	pub build_args: Option<HashMap<String, String>>,
	pub dockerfile: Option<String>,
	pub image: Option<String>,
}

pub struct DeployOutput {
	pub image_id: Uuid,
}

/// Builds image if not specified and returns the image ID.
///
/// The image ID is chosen in order of priority:
///
/// - `dockerfile` Build the Dockerfile
/// - `image` Upload a prebuilt image
///
/// If none are true, `None` is returned.
pub async fn deploy(ctx: &Ctx, task: TaskCtx, opts: DeployOpts) -> GlobalResult<DeployOutput> {
	task.log_stdout("[Deploying Game Server]");

	let image_id = if let Some(dockerfile) = &opts.dockerfile {
		let push_output = build_and_push(
			ctx,
			task.clone(),
			&Path::new(&opts.build_dir),
			&BuildPushOpts {
				dockerfile: dockerfile.clone(),
				name: Some(opts.display_name.clone()),
				build_args: Some(
					opts.build_args
						.iter()
						.flatten()
						.map(|(k, v)| format!("{k}={v}"))
						.collect(),
				),
			},
		)
		.await?;

		task.log_stdout(format!("[Created Build] {}", push_output.image_id));

		push_output.image_id
	} else if let Some(docker_image) = opts.image.as_ref() {
		let push_output = push(
			ctx,
			task.clone(),
			&PushOpts {
				tag: docker_image.clone(),
				name: Some(opts.display_name.to_string()),
			},
		)
		.await?;

		task.log_stdout(format!("[Created Build] {}", push_output.image_id));

		push_output.image_id
	} else {
		bail!("must specify dockerfile or image tag")
	};

	Ok(DeployOutput { image_id })
}

pub struct BuildPushOpts {
	/// Path to Dockerfile
	pub dockerfile: String,

	/// Name of the image
	pub name: Option<String>,

	/// Docker build args
	pub build_args: Option<Vec<String>>,
}

/// Build and push a Dockerfile.
pub async fn build_and_push(
	ctx: &Ctx,
	task: TaskCtx,
	current_dir: &Path,
	push_opts: &BuildPushOpts,
) -> GlobalResult<docker::push::PushOutput> {
	// Build image
	let build_kind = BuildKind::from_env().await?;
	let build_compression = BuildCompression::from_env(&build_kind).await?;
	let build_output = docker::build::build_image(
		ctx,
		task.clone(),
		current_dir,
		&Path::new(&push_opts.dockerfile),
		build_kind,
		build_compression,
		push_opts.build_args.as_ref().map(|x| x.as_slice()),
	)
	.await?;

	// Upload build
	docker::push::push_tar(
		ctx,
		task.clone(),
		&docker::push::PushOpts {
			path: build_output.path.to_owned(),
			tag: build_output.tag,
			name: push_opts.name.clone(),
			kind: build_kind,
			compression: build_compression,
		},
	)
	.await
}

pub struct PushOpts {
	pub tag: String,
	pub name: Option<String>,
}

/// Push an image that's already built.
pub async fn push(
	ctx: &Ctx,
	task: TaskCtx,
	push_opts: &PushOpts,
) -> GlobalResult<docker::push::PushOutput> {
	// Re-tag image with unique tag
	let unique_image_tag = generate_unique_image_tag();
	let mut tag_cmd = shell_cmd("docker");
	tag_cmd
		.arg("image")
		.arg("tag")
		.arg(&push_opts.tag)
		.arg(&unique_image_tag);
	cmd::execute_docker_cmd_silent(tag_cmd, "failed to tag Docker image").await?;

	// Archive image
	let build_kind = BuildKind::from_env().await?;
	let build_compression = BuildCompression::from_env(&build_kind).await?;
	let archive_path = docker::archive::create_archive(
		task.clone(),
		&unique_image_tag,
		build_kind,
		build_compression,
	)
	.await?;

	docker::push::push_tar(
		ctx,
		task.clone(),
		&docker::push::PushOpts {
			path: archive_path.to_owned(),
			tag: unique_image_tag,
			name: push_opts.name.clone(),
			kind: build_kind,
			compression: build_compression,
		},
	)
	.await
}
