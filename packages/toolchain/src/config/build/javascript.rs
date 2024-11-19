use serde::{Deserialize, Serialize};

use super::Compression;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct Build {
	pub path: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mode: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub command: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct Unstable {
	pub compression: Compression,
}
