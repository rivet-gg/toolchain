use serde::{Deserialize, Serialize};

use super::Compression;

// TODO: Add back `deny_unknown_fields` after https://github.com/serde-rs/serde/issues/1600
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Build {
	#[serde(flatten)]
	pub bundler: Bundler,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub unstable: Option<Unstable>,
}

impl Build {
	pub fn unstable(&self) -> Unstable {
		self.unstable.clone().unwrap_or_default()
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "bundler", deny_unknown_fields)]
pub enum Bundler {
	// Deno(DenoBuildMethod),
	None(NoBuildMethod),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "bundler", deny_unknown_fields)]
pub struct NoBuildMethod {
	pub index_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "bundler", deny_unknown_fields)]
pub struct DenoBuildMethod {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub build_path: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub import_map: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub deno: Option<Deno>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bundle: Option<Bundle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct Deno {
	pub config: String,
	pub no_lock: bool,
	pub no_remote: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct Bundle {
	pub minify: bool,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct Unstable {
	// TODO(RVT-4127): Add compression support
	// pub compression: Option<Compression>,
}

impl Unstable {
	// TODO(RVT-4127): Add compression support
	pub fn compression(&self) -> Compression {
		Compression::None
	}
}
