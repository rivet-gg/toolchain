use std::{collections::HashMap, path::PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::{
	fs,
	sync::{Mutex, OnceCell},
};

use super::paths;

/// Configuration stored globally on the file system.
#[derive(Default, Serialize, Deserialize)]
pub struct GlobalConfig {
	/// Store project meta by the absolute path to the project.
	project_roots: HashMap<PathBuf, ProjectMeta>,
}

/// Config stored in .rivet/config.yaml. Used to store persistent data.
#[derive(Default, Serialize, Deserialize)]
pub struct ProjectMeta {
	#[serde(default)]
	pub cluster: Cluster,
	#[serde(default)]
	pub telemetry: Telemetry,
	#[serde(default)]
	pub tokens: Tokens,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Cluster {
	#[serde(default)]
	pub api_endpoint: Option<String>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Telemetry {
	#[serde(default)]
	pub disabled: bool,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Tokens {
	#[serde(default)]
	pub cloud: Option<String>,
}

static SINGLETON: OnceCell<Mutex<GlobalConfig>> = OnceCell::const_new();

/// Gets the global config instance.
///
/// Use `read` to read properties from the config.
async fn get_or_init() -> Result<&'static Mutex<GlobalConfig>> {
	SINGLETON
		.get_or_try_init::<anyhow::Error, _, _>(|| async {
			let path = paths::global_config_file()?;

			let config = match fs::read_to_string(&path).await {
				Ok(config) => serde_yaml::from_str(&config).map_err(Into::<anyhow::Error>::into)?,
				Err(err) if err.kind() == std::io::ErrorKind::NotFound => GlobalConfig::default(),
				Err(err) => return Err(err.into()),
			};

			Result::Ok(Mutex::new(config))
		})
		.await
}

/// Writes the config to the file system.
///
/// Use `mutate` to make changes to the config publicly.
async fn write(config: &GlobalConfig) -> Result<()> {
	fs::create_dir_all(paths::global_config_dir()?).await?;
	fs::write(paths::global_config_file()?, serde_yaml::to_string(config)?).await?;

	Ok(())
}

/// Reads from the global config.
pub async fn try_read_global<F: FnOnce(&GlobalConfig) -> Result<T>, T>(cb: F) -> Result<T> {
	let singleton = get_or_init().await?;
	let mut lock = singleton.lock().await;

	// Fetch value
	let value = cb(&mut *lock)?;

	Ok(value)
}

/// Reads from the project meta.
///
/// If project meta does not exist, returns the default value.
pub async fn try_read_project<F: FnOnce(&ProjectMeta) -> Result<T>, T>(cb: F) -> Result<T> {
	let project_root = paths::project_root()?;
	try_read_global(|config| {
		if let Some(project_config) = config.project_roots.get(&project_root) {
			cb(project_config)
		} else {
			cb(&ProjectMeta::default())
		}
	})
	.await
}

/// Non-failable version of `try_read_project`.
pub async fn read_project<F: FnOnce(&ProjectMeta) -> T, T>(cb: F) -> Result<T> {
	try_read_project(|x| Ok(cb(x))).await
}

pub async fn try_mutate_global<F: FnOnce(&mut GlobalConfig) -> Result<()>>(cb: F) -> Result<()> {
	let singleton = get_or_init().await?;
	let mut lock = singleton.lock().await;

	// Mutate the config
	cb(&mut *lock)?;

	// Write new changes
	write(&*lock).await?;

	Ok(())
}

/// Mutates the project meta.
///
/// If the project meta does not exist, a default one will be inserted and modified.
pub async fn try_mutate_project<F: FnOnce(&mut ProjectMeta) -> Result<()>>(cb: F) -> Result<()> {
	let project_root = paths::project_root()?;
	try_mutate_global(|config| {
		let project_config = config.project_roots.entry(project_root).or_default();
		cb(project_config)
	})
	.await
}

/// Non-failable version of `try_mutate_project`.
pub async fn mutate_project<F: FnOnce(&mut ProjectMeta) -> ()>(cb: F) -> Result<()> {
	try_mutate_project(|x| Ok(cb(x))).await
}
