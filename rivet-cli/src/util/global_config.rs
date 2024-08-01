use std::{collections::HashMap, path::PathBuf};

use toolchain_core::rivet_api::models;
use global_error::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::{
	fs,
	sync::{Mutex, OnceCell},
};
use uuid::Uuid;

use super::paths;

/// Configuration stored globally on the file system.
#[derive(Default, Serialize, Deserialize)]
pub struct GlobalConfig {
	/// Store project meta by the absolute path to the project.
	pub project_roots: HashMap<PathBuf, ProjectMeta>,
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
	#[serde(default)]
	pub opengb: OpenGb,
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
	/// Cloud token used to authenticate all API requests.
	///
	/// If none provided, will be prompted for token on `rivet init`.
	#[serde(default)]
	pub cloud: Option<String>,

	/// List of cached public namespace tokens.
	#[serde(default)]
	pub public_namespace: Vec<PublicNamespaceToken>,

	/// List of cached development tokens. Before creating a new token, this list will be checked
	/// for an existing token.
	#[serde(default)]
	pub development: Vec<DevelopmentToken>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct PublicNamespaceToken {
	pub namespace_name_id: String,
	pub token: String,
}

#[derive(Default, Serialize, Deserialize)]
pub struct DevelopmentToken {
	pub namespace_name_id: String,
	pub hostname: String,
	pub ports: HashMap<String, models::CloudMatchmakerDevelopmentPort>,
	pub token: String,
}

#[derive(Default, Serialize, Deserialize)]
pub struct OpenGb {
	#[serde(default)]
	pub projects: HashMap<Uuid, OpenGbProject>,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct OpenGbProject {
	#[serde(default)]
	pub environments: HashMap<Uuid, OpenGbEnv>,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct OpenGbEnv {
	#[serde(default)]
	pub url: Option<String>,
}

static SINGLETON: OnceCell<Mutex<GlobalConfig>> = OnceCell::const_new();

/// Gets the global config instance.
///
/// Use `read` to read properties from the config.
async fn get_or_init() -> GlobalResult<&'static Mutex<GlobalConfig>> {
	SINGLETON
		.get_or_try_init::<GlobalError, _, _>(|| async {
			let path = paths::global_config_file()?;

			let config = match fs::read_to_string(&path).await {
				Ok(config) => serde_yaml::from_str(&config).map_err(Into::<GlobalError>::into)?,
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
async fn write(config: &GlobalConfig) -> GlobalResult<()> {
	fs::create_dir_all(paths::global_config_dir()?).await?;
	fs::write(paths::global_config_file()?, serde_yaml::to_string(config)?).await?;

	Ok(())
}

/// Reads from the global config.
pub async fn try_read_global<F: FnOnce(&GlobalConfig) -> GlobalResult<T>, T>(
	cb: F,
) -> GlobalResult<T> {
	let singleton = get_or_init().await?;
	let mut lock = singleton.lock().await;

	// Fetch value
	let value = cb(&mut *lock)?;

	Ok(value)
}

/// Reads from the project meta.
///
/// If project meta does not exist, returns the default value.
pub async fn try_read_project<F: FnOnce(&ProjectMeta) -> GlobalResult<T>, T>(
	cb: F,
) -> GlobalResult<T> {
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
pub async fn read_project<F: FnOnce(&ProjectMeta) -> T, T>(cb: F) -> GlobalResult<T> {
	try_read_project(|x| Ok(cb(x))).await
}

pub async fn try_mutate_global<F: FnOnce(&mut GlobalConfig) -> GlobalResult<T>, T>(
	cb: F,
) -> GlobalResult<T> {
	let singleton = get_or_init().await?;
	let mut lock = singleton.lock().await;

	// Mutate the config
	let res = cb(&mut *lock)?;

	// Write new changes
	write(&*lock).await?;

	Ok(res)
}

/// Mutates the project meta.
///
/// If the project meta does not exist, a default one will be inserted and modified.
pub async fn try_mutate_project<F: FnOnce(&mut ProjectMeta) -> GlobalResult<T>, T>(
	cb: F,
) -> GlobalResult<T> {
	let project_root = paths::project_root()?;
	try_mutate_global(|config| {
		let project_config = config.project_roots.entry(project_root).or_default();
		cb(project_config)
	})
	.await
}

/// Non-failable version of `try_mutate_project`.
pub async fn mutate_project<F: FnOnce(&mut ProjectMeta) -> T, T>(cb: F) -> GlobalResult<T> {
	try_mutate_project(|x| Ok(cb(x))).await
}
