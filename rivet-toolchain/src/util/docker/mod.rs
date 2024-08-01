pub mod archive;
pub mod build;
pub mod push;
pub mod users;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Copy, Clone, Serialize, Deserialize, strum::AsRefStr)]
pub enum BuildCompression {
	/// No compression.
	#[strum(serialize = "none")]
	None,

	/// LZ4 compression. Fast compression optimized for fast lobby start times.
	#[strum(serialize = "lz4")]
	Lz4,
}

impl BuildCompression {
	pub fn default_from_build_kind(build_kind: BuildKind) -> Self {
		match build_kind {
			BuildKind::DockerImage => BuildCompression::None,
			BuildKind::OciBundle => BuildCompression::Lz4,
		}
	}
}

#[derive(Copy, Clone, Serialize, Deserialize, strum::AsRefStr)]
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

/// Generates a unique image tag for the image being pushed or built.
pub fn generate_unique_image_tag() -> String {
	format!("rivet-game:{}", Uuid::new_v4())
}
