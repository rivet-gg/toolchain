use anyhow::*;
use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf};

use crate::paths;

/// Partial serde struct representing data we need to read from `project_manifest.json`.
///
/// See packages/backend/toolchain/build/project_manifest.ts
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
	pub sdks: Vec<Sdk>,
	pub modules: HashMap<String, Module>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sdk {
	pub target: String,
	pub output: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Module {
	pub config: ModuleConfig,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModuleConfig {
	pub name: String,
}

pub fn path(data_dir: &PathBuf, data_type: paths::BackendDataType) -> Result<PathBuf> {
	Ok(paths::backend_data_dir(data_dir, data_type)?.join("project_manifest.json"))
}

/// Reads the `project_manifest.json` from the filesystem.
pub async fn read(data_dir: &PathBuf, data_type: paths::BackendDataType) -> Result<Meta> {
	// Read meta
	let manifest_path = path(data_dir, data_type)?;
	tokio::task::block_in_place(|| {
		let file = std::fs::File::open(&manifest_path)?;
		let meta = serde_json::from_reader::<_, Meta>(&file)?;
		Ok(meta)
	})
}
