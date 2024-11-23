use anyhow::*;
use std::path::Path;
use uuid::Uuid;

use crate::{
	config, paths,
	project::environment::TEMPEnvironment,
	toolchain_ctx::ToolchainCtx,
	util::{
		cmd::{self, shell_cmd},
		docker::{self, generate_unique_image_tag},
		task,
	},
};

pub struct DeployBuildOpts {
	pub env: TEMPEnvironment,
	pub config: config::Config,
	pub build_config: config::build::docker::Build,
	pub version_name: String,
}

/// Builds image if not specified and returns the build ID.
pub async fn build_and_upload(
	ctx: &ToolchainCtx,
	task: task::TaskCtx,
	opts: DeployBuildOpts,
) -> Result<Uuid> {
	task.log("[Build]");

	let project_root = paths::project_root()?;

	// Determine build attributes
	let build_config_unstable = opts.build_config.unstable();
	let bundle = build_config_unstable.bundle();
	let compression = build_config_unstable
		.compression
		.unwrap_or_else(|| config::build::Compression::default_from_bundle_kind(bundle));

	// Deploy Docker build
	let build_id = if let Some(image) = &opts.build_config.image {
		let push_output = docker_push(
			ctx,
			task.clone(),
			&DockerPushOpts {
				config: opts.config.clone(),
				env: opts.env,
				name: Some(opts.version_name.to_string()),
				docker_tag: image.clone(),
				bundle,
				compression,
				allow_root: build_config_unstable.allow_root(),
			},
		)
		.await?;

		task.log(format!("[Created Build] {}", push_output.build_id));

		push_output.build_id
	} else {
		let dockerfile = opts
			.build_config
			.dockerfile
			.unwrap_or_else(|| "Dockerfile".to_string());

		let path = opts
			.build_config
			.build_path
			.as_ref()
			.map(|x| x.as_str())
			.unwrap_or(".");

		let push_output = docker_build_and_push(
			ctx,
			task.clone(),
			&project_root.join(path),
			&DockerBuildPushOpts {
				config: opts.config.clone(),
				env: opts.env.clone(),
				dockerfile: dockerfile.clone(),
				name: Some(opts.version_name.clone()),
				build_args: Some(
					opts.build_config
						.build_args
						.iter()
						.flatten()
						.map(|(k, v)| format!("{k}={v}"))
						.collect(),
				),
				build_target: opts.build_config.build_target.clone(),
				build_method: build_config_unstable.build_method(),
				bundle,
				compression,
				allow_root: build_config_unstable.allow_root(),
			},
		)
		.await?;

		task.log(format!("[Created Build] {}", push_output.build_id));

		push_output.build_id
	};

	Ok(build_id)
}

pub struct DockerBuildPushOpts {
	pub config: config::Config,
	pub env: TEMPEnvironment,
	pub name: Option<String>,

	/// Path to Dockerfile
	pub dockerfile: String,

	/// Docker build args
	pub build_args: Option<Vec<String>>,

	/// Target build stage to build.
	pub build_target: Option<String>,

	pub build_method: config::build::docker::BuildMethod,
	pub bundle: config::build::docker::BundleKind,
	pub compression: config::build::Compression,
	pub allow_root: bool,
}

/// Build and push a Dockerfile.
pub async fn docker_build_and_push(
	ctx: &ToolchainCtx,
	task: task::TaskCtx,
	current_dir: &Path,
	push_opts: &DockerBuildPushOpts,
) -> Result<docker::push::PushOutput> {
	// Build image
	let build_output = docker::build::build_image(
		ctx,
		task.clone(),
		current_dir,
		&Path::new(&push_opts.dockerfile),
		push_opts.build_method,
		push_opts.bundle,
		push_opts.compression,
		push_opts.build_args.as_ref().map(|x| x.as_slice()),
		push_opts.build_target.as_ref().map(String::as_str),
		push_opts.allow_root,
	)
	.await?;

	// Upload build
	docker::push::push_tar(
		ctx,
		task.clone(),
		&docker::push::PushOpts {
			config: push_opts.config.clone(),
			env: push_opts.env.clone(),
			path: build_output.path.to_owned(),
			docker_tag: build_output.tag,
			name: push_opts.name.clone(),
			bundle: push_opts.bundle,
			compression: push_opts.compression,
		},
	)
	.await
}

pub struct DockerPushOpts {
	pub config: config::Config,
	pub env: TEMPEnvironment,
	pub name: Option<String>,

	pub docker_tag: String,

	pub bundle: config::build::docker::BundleKind,
	pub compression: config::build::Compression,
	pub allow_root: bool,
}

/// Push an image that's already built.
pub async fn docker_push(
	ctx: &ToolchainCtx,
	task: task::TaskCtx,
	push_opts: &DockerPushOpts,
) -> Result<docker::push::PushOutput> {
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
		push_opts.bundle,
		push_opts.compression,
		push_opts.allow_root,
	)
	.await?;

	docker::push::push_tar(
		ctx,
		task.clone(),
		&docker::push::PushOpts {
			config: push_opts.config.clone(),
			env: push_opts.env.clone(),
			path: archive_path.to_owned(),
			docker_tag: unique_image_tag,
			name: push_opts.name.clone(),
			bundle: push_opts.bundle,
			compression: push_opts.compression,
		},
	)
	.await
}
