use anyhow::*;
use rivet_api::apis;
use serde::Serialize;
use std::{collections::HashMap, path::Path};
use uuid::Uuid;

use crate::{
	config,
	game::TEMPEnvironment,
	game_server, paths,
	toolchain_ctx::ToolchainCtx,
	util::{
		cmd::{self, shell_cmd},
		docker::{self, generate_unique_image_tag, BuildCompression},
		task,
	},
};

pub struct DeployOpts {
	pub env: TEMPEnvironment,
	pub build_dir: String,
	// pub build_args: Option<HashMap<String, String>>,
	// pub dockerfile: Option<String>,
	// pub image: Option<String>,
}

#[derive(Serialize)]
pub struct DeployOutput {
	pub image_id: Uuid,
	pub version_name: String,
}

/// Builds image if not specified and returns the image ID.
///
/// The image ID is chosen in order of priority:
///
/// - `dockerfile` Build the Dockerfile
/// - `image` Upload a prebuilt image
///
/// If none are true, `None` is returned.
pub async fn deploy(
	ctx: &ToolchainCtx,
	task: task::TaskCtx,
	opts: DeployOpts,
) -> Result<DeployOutput> {
	task.log("[Deploying Game Server]");

	let deploy_config =
		config::settings::try_read(&paths::data_dir()?, |x| Ok(x.game_server.deploy.clone()))
			.await?;

	// Reserve image name
	let reserve_res = apis::cloud_games_versions_api::cloud_games_versions_reserve_version_name(
		&ctx.openapi_config_cloud,
		&ctx.game_id,
	)
	.await?;
	let version_name = reserve_res.version_display_name;

	// Build tags
	//
	// ## version
	//
	// Unique ident for this build. Used for figuring out which server to start when
	// passing dynamic version from client.
	//
	// ## active
	//
	// If this build can be used to start a new server. Remove this tag to disable client-side
	// joining this version.
	//
	// This is not exclusive.
	//
	// ## latest
	//
	// Indicates the latest build to use for this environment. Used if not providing a client-side
	// version.
	let tags = HashMap::from([
		(
			game_server::VERSION_BUILD_TAG.to_string(),
			version_name.clone(),
		),
		(
			game_server::ENABLED_BUILD_TAG.to_string(),
			"true".to_string(),
		),
		(
			game_server::CURRENT_BUILD_TAG.to_string(),
			"true".to_string(),
		),
	]);
	let exclusive_tags = vec![
		game_server::VERSION_BUILD_TAG.to_string(),
		game_server::CURRENT_BUILD_TAG.to_string(),
	];

	// Deploy Docker image
	let image_id = if let Some(docker_image) = deploy_config.docker_image.as_ref() {
		let push_output = push(
			ctx,
			task.clone(),
			&PushOpts {
				env_id: opts.env.id,
				name: Some(version_name.to_string()),
				tags,
				exclusive_tags,
				docker_tag: docker_image.clone(),
			},
		)
		.await?;

		task.log(format!("[Created Build] {}", push_output.image_id));

		push_output.image_id
	} else {
		let dockerfile = deploy_config
			.dockerfile_path
			.unwrap_or_else(|| "game_server.Dockerfile".to_string());

		let push_output = build_and_push(
			ctx,
			task.clone(),
			&Path::new(&opts.build_dir),
			&BuildPushOpts {
				env_id: opts.env.id,
				tags,
				exclusive_tags,
				dockerfile: dockerfile.clone(),
				name: Some(version_name.clone()),
				build_args: Some(
					deploy_config
						.build_args
						.iter()
						.flatten()
						.map(|(k, v)| format!("{k}={v}"))
						.collect(),
				),
			},
		)
		.await?;

		task.log(format!("[Created Build] {}", push_output.image_id));

		push_output.image_id
	};

	Ok(DeployOutput {
		image_id,
		version_name,
	})
}

pub struct BuildPushOpts {
	pub env_id: Uuid,
	pub name: Option<String>,
	pub tags: HashMap<String, String>,
	pub exclusive_tags: Vec<String>,

	/// Path to Dockerfile
	pub dockerfile: String,

	/// Docker build args
	pub build_args: Option<Vec<String>>,
}

/// Build and push a Dockerfile.
pub async fn build_and_push(
	ctx: &ToolchainCtx,
	task: task::TaskCtx,
	current_dir: &Path,
	push_opts: &BuildPushOpts,
) -> Result<docker::push::PushOutput> {
	let (build_kind, build_compression) = config::settings::try_read(&paths::data_dir()?, |x| {
		Ok((
			x.game_server.deploy.build_kind.clone(),
			x.game_server.deploy.build_compression.clone(),
		))
	})
	.await?;
	let build_compression =
		build_compression.unwrap_or_else(|| BuildCompression::default_from_build_kind(build_kind));

	// Build image
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
			env_id: push_opts.env_id,
			path: build_output.path.to_owned(),
			tags: push_opts.tags.clone(),
			exclusive_tags: push_opts.exclusive_tags.clone(),
			docker_tag: build_output.tag,
			name: push_opts.name.clone(),
			kind: build_kind,
			compression: build_compression,
		},
	)
	.await
}

pub struct PushOpts {
	pub env_id: Uuid,
	pub name: Option<String>,
	pub tags: HashMap<String, String>,
	pub exclusive_tags: Vec<String>,

	pub docker_tag: String,
}

/// Push an image that's already built.
pub async fn push(
	ctx: &ToolchainCtx,
	task: task::TaskCtx,
	push_opts: &PushOpts,
) -> Result<docker::push::PushOutput> {
	let (build_kind, build_compression) = config::settings::try_read(&paths::data_dir()?, |x| {
		Ok((
			x.game_server.deploy.build_kind.clone(),
			x.game_server.deploy.build_compression.clone(),
		))
	})
	.await?;
	let build_compression =
		build_compression.unwrap_or_else(|| BuildCompression::default_from_build_kind(build_kind));

	// Re-tag image with unique tag
	let unique_image_tag = generate_unique_image_tag();
	let mut tag_cmd = shell_cmd("docker");
	tag_cmd
		.arg("image")
		.arg("tag")
		.arg(&push_opts.docker_tag)
		.arg(&unique_image_tag);
	cmd::execute_docker_cmd_silent(tag_cmd, "failed to tag Docker image").await?;

	// Archive image
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
			env_id: push_opts.env_id,
			path: archive_path.to_owned(),
			tags: push_opts.tags.clone(),
			exclusive_tags: push_opts.exclusive_tags.clone(),
			docker_tag: unique_image_tag,
			name: push_opts.name.clone(),
			kind: build_kind,
			compression: build_compression,
		},
	)
	.await
}
