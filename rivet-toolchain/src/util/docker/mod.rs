pub mod archive;
pub mod build;
pub mod push;
pub mod users;

use global_error::prelude::*;
use std::str::FromStr;
use uuid::Uuid;

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

/// Generates a unique image tag for the image being pushed or built.
pub fn generate_unique_image_tag() -> String {
	format!("rivet-game:{}", Uuid::new_v4())
}
