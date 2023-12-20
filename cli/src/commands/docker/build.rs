use std::{path::Path, str::FromStr};

use anyhow::Result;
use tokio::process::Command;

use crate::util::{cmd, term};

#[derive(strum::EnumString)]
pub enum DockerBuildMethod {
	/// Create & use a Buildx builder on this machine. Required for cross-platform compliation.
	#[strum(serialize = "buildx")]
	Buildx,

	/// Use the native Docker build command. Only used if Buildx is not available.
	#[strum(serialize = "native")]
	Native,
}

impl Default for DockerBuildMethod {
	fn default() -> Self {
		Self::Buildx
	}
}

impl DockerBuildMethod {
	pub async fn from_env() -> Result<Self> {
		// Determine build method from env
		if let Some(method) = std::env::var("_RIVET_DOCKER_BUILD_METHOD")
			.ok()
			.and_then(|x| DockerBuildMethod::from_str(&x).ok())
		{
			Ok(method)
		} else {
			// Validate that Buildx is installed
			let mut buildx_version_cmd = Command::new("docker");
			buildx_version_cmd.args(&["buildx", "version"]);
			let buildx_version =
				cmd::execute_docker_cmd_silent_fallible(buildx_version_cmd).await?;

			if buildx_version.status.success() {
				Ok(DockerBuildMethod::Buildx)
			} else {
				println!("Docker Buildx not installed. Falling back to native build method.\n\nPlease install Buildx here: https://github.com/docker/buildx#installing");
				Ok(DockerBuildMethod::Native)
			}
		}
	}
}

pub struct BuildImageOutput {
	pub tag: String,
	pub path: tempfile::TempPath,
}

/// Builds an image and archives it to a path.
pub async fn build_image(
	dockerfile: &Path,
	build_kind: super::BuildKind,
	build_compression: super::BuildCompression,
) -> Result<BuildImageOutput> {
	// if docker.image_id.is_none() {
	// if let Some(dockerfile) = docker.dockerfile.as_ref() {
	let build_method = DockerBuildMethod::from_env().await?;

	eprintln!();
	let buildx_info = match build_method {
		DockerBuildMethod::Native => " (with native)",
		DockerBuildMethod::Buildx => " (with buildx)",
	};
	term::status::info(
		"Building Image",
		format!("{}{buildx_info}", dockerfile.display()),
	);

	// Build image
	let image_tag = super::generate_unique_image_tag();
	match build_method {
		DockerBuildMethod::Native => {
			let mut build_cmd = Command::new("docker");
			build_cmd
				.arg("build")
				.arg("--file")
				.arg(dockerfile)
				.arg("--tag")
				.arg(&image_tag)
				.arg("--platform")
				.arg("linux/amd64")
				.arg(".");
			cmd::execute_docker_cmd(build_cmd, "Docker image failed to build").await?;
		}
		DockerBuildMethod::Buildx => {
			let builder_name = "rivet_cli";

			// Determine if needs to create a new builder
			let mut inspect_cmd = Command::new("docker");
			inspect_cmd.arg("buildx").arg("inspect").arg(builder_name);
			let inspect_output = cmd::execute_docker_cmd_silent_fallible(inspect_cmd).await?;

			if !inspect_output.status.success()
				&& String::from_utf8(inspect_output.stderr.clone())?
					.contains(&format!("no builder \"{builder_name}\" found"))
			{
				// Create new builder
				let mut build_cmd = Command::new("docker");
				build_cmd
					.arg("buildx")
					.arg("create")
					.arg("--name")
					.arg(builder_name)
					.arg("--driver")
					.arg("docker-container")
					.arg("--platform")
					.arg("linux/amd64");
				cmd::execute_docker_cmd(build_cmd, "Failed to create Docker Buildx builder")
					.await?;
			} else {
				// Builder exists

				cmd::error_for_output_failure(
					&inspect_output,
					"Failed to inspect Docker Buildx runner",
				)?;
			}

			// Build image
			let mut build_cmd = Command::new("docker");
			build_cmd
				.arg("buildx")
				.arg("build")
				.arg("--builder")
				.arg(builder_name)
				.arg("--platform")
				.arg("linux/amd64")
				.arg("--file")
				.arg(dockerfile)
				.arg("--tag")
				.arg(&image_tag)
				.arg("--output")
				.arg("type=docker")
				.arg(".");
			cmd::execute_docker_cmd(build_cmd, "Docker image failed to build").await?;
		}
	}

	// Build archive
	let build_tar_path =
		super::archive::create_archive(&image_tag, build_kind, build_compression).await?;

	// Clean up image from the registry
	let mut remove_img_cmd = Command::new("docker");
	remove_img_cmd
		.arg("image")
		.arg("rm")
		.arg("--force")
		.arg(&image_tag);
	cmd::execute_docker_cmd_silent_fallible(remove_img_cmd).await?;

	Ok(BuildImageOutput {
		tag: image_tag,
		path: build_tar_path,
	})
}
