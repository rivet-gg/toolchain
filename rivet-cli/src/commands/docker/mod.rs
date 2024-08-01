mod archive;
mod build;
pub mod push;

use clap::Parser;
use global_error::prelude::*;
use std::{path::Path, str::FromStr};
use uuid::Uuid;

use crate::util::{cmd, struct_fmt};

#[derive(Parser)]
pub enum SubCommand {
	/// Pushes a image to Rivet so it can be used in a version
	Push(PushOpts),
	/// Builds and pushes a image to Rivet so it can be used in a version
	BuildPush(BuildPushOpts),
}

#[derive(Parser)]
pub struct PushOpts {
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
pub struct BuildPushOpts {
	/// Path to Dockerfile
	#[clap(long)]
	pub dockerfile: String,

	/// Name of the image
	#[clap(long)]
	pub name: Option<String>,

	/// Docker build args
	#[clap(long = "build-arg")]
	pub build_args: Option<Vec<String>>,

	#[clap(long, value_parser)]
	pub format: Option<struct_fmt::Format>,
}

impl SubCommand {
	pub async fn execute(&self, ctx: &toolchain_core::Ctx) -> GlobalResult<()> {
		match self {
			SubCommand::Push(push_opts) => {
				let output = push(ctx, push_opts).await?;
				struct_fmt::print_opt(push_opts.format.as_ref(), &output)?;
				Ok(())
			}
			SubCommand::BuildPush(push_opts) => {
				let output = build_and_push(ctx, push_opts).await?;
				struct_fmt::print_opt(push_opts.format.as_ref(), &output)?;
				Ok(())
			}
		}
	}
}

#[derive(Copy, Clone, strum::EnumString, strum::AsRefStr)]
pub enum BuildKind {
	/// Legacy option. Docker image archive output from `docker save`. Slower lobby start
	/// times.
	#[strum(serialize = "docker-image")]
	DockerImage,

	/// OCI bundle archive derived from a generated Docker image. Optimized for fast lobby start
	/// times.
	#[strum(serialize = "oci-bundle")]
	OciBundle,
}

impl Default for BuildKind {
	fn default() -> Self {
		Self::OciBundle
	}
}

impl BuildKind {
	pub async fn from_env() -> GlobalResult<BuildKind> {
		// Determine build method from env
		if let Some(method) = std::env::var("_RIVET_BUILD_KIND")
			.ok()
			.and_then(|x| BuildKind::from_str(&x).ok())
		{
			Ok(method)
		} else {
			Ok(BuildKind::OciBundle)
		}
	}
}

#[derive(Copy, Clone, strum::EnumString, strum::AsRefStr)]
pub enum BuildCompression {
	/// No compression.
	#[strum(serialize = "none")]
	None,

	/// LZ4 compression. Fast compression optimized for fast lobby start times.
	#[strum(serialize = "lz4")]
	Lz4,
}

impl Default for BuildCompression {
	fn default() -> Self {
		Self::Lz4
	}
}

impl BuildCompression {
	pub async fn from_env(kind: &BuildKind) -> GlobalResult<BuildCompression> {
		// Determine build method from env
		if let Some(method) = std::env::var("_RIVET_BUILD_COMPRESSION")
			.ok()
			.and_then(|x| BuildCompression::from_str(&x).ok())
		{
			Ok(method)
		} else {
			Ok(match kind {
				BuildKind::DockerImage => BuildCompression::None,
				BuildKind::OciBundle => BuildCompression::Lz4,
			})
		}
	}
}

pub async fn push(ctx: &toolchain_core::Ctx, push_opts: &PushOpts) -> GlobalResult<push::PushOutput> {
	// Re-tag image with unique tag
	let unique_image_tag = generate_unique_image_tag();
	let mut tag_cmd = tokio::process::Command::new("docker");
	tag_cmd
		.arg("image")
		.arg("tag")
		.arg(&push_opts.tag)
		.arg(&unique_image_tag);
	cmd::execute_docker_cmd_silent(tag_cmd, "failed to tag Docker image").await?;

	// Archive image
	let build_kind = BuildKind::from_env().await?;
	let build_compression = BuildCompression::from_env(&build_kind).await?;
	let archive_path =
		archive::create_archive(&unique_image_tag, build_kind, build_compression).await?;

	push::push_tar(
		ctx,
		&push::PushOpts {
			path: archive_path.to_owned(),
			tag: unique_image_tag,
			name: push_opts.name.clone(),
			kind: build_kind,
			compression: build_compression,
			format: push_opts.format.clone(),
		},
	)
	.await
}

pub async fn build_and_push(
	ctx: &toolchain_core::Ctx,
	push_opts: &BuildPushOpts,
) -> GlobalResult<push::PushOutput> {
	// Build image
	let build_kind = BuildKind::from_env().await?;
	let build_compression = BuildCompression::from_env(&build_kind).await?;
	let build_output = build::build_image(
		ctx,
		&Path::new(&push_opts.dockerfile),
		build_kind,
		build_compression,
		push_opts.build_args.as_ref().map(|x| x.as_slice()),
	)
	.await?;

	// Upload build
	push::push_tar(
		ctx,
		&push::PushOpts {
			path: build_output.path.to_owned(),
			tag: build_output.tag,
			name: push_opts.name.clone(),
			kind: build_kind,
			compression: build_compression,
			format: push_opts.format.clone(),
		},
	)
	.await
}

/// Generates a unique image tag for the image being pushed or built.
pub fn generate_unique_image_tag() -> String {
	format!("rivet-game:{}", Uuid::new_v4())
}
