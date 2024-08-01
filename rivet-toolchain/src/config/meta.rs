use std::{collections::HashMap, path::PathBuf};

use global_error::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::{
	fs,
	sync::{Mutex, OnceCell},
};
use uuid::Uuid;

use crate::paths;

/// Configuration stored globally on the file system.
#[derive(Default, Serialize, Deserialize)]
pub struct Meta {
	/// Store project meta by the absolute path to the project.
	pub projects: HashMap<PathBuf, ProjectMeta>,
}

/// Config stored in .rivet/meta.json. Used to store persistent data, such as tokens & cache.
#[derive(Serialize, Deserialize)]
pub struct ProjectMeta {
	pub cluster: Cluster,
	pub tokens: Tokens,
	pub opengb: OpenGb,
}

impl ProjectMeta {
	fn new(api_endpoint: String, cloud_token: String) -> Self {
		ProjectMeta {
			cluster: Cluster { api_endpoint },
			tokens: Tokens { cloud: cloud_token },
			opengb: OpenGb::default(),
		}
	}
}

#[derive(Default, Serialize, Deserialize)]
pub struct Cluster {
	pub api_endpoint: String,
}

#[derive(Serialize, Deserialize)]
pub struct Tokens {
	/// Cloud token used to authenticate all API requests.
	pub cloud: String,
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

static SINGLETON: OnceCell<Mutex<Meta>> = OnceCell::const_new();

/// Gets the global config instance.
///
/// Use `read` to read properties from the config.
async fn get_or_init() -> GlobalResult<&'static Mutex<Meta>> {
	SINGLETON
		.get_or_try_init::<GlobalError, _, _>(|| async {
			let path = paths::meta_config_file()?;

			let config = match fs::read_to_string(&path).await {
				Ok(config) => serde_json::from_str(&config).map_err(Into::<GlobalError>::into)?,
				Err(err) if err.kind() == std::io::ErrorKind::NotFound => Meta::default(),
				Err(err) => return Err(err.into()),
			};

			Result::Ok(Mutex::new(config))
		})
		.await
}

/// Writes the config to the file system.
///
/// Use `mutate` to make changes to the config publicly.
async fn write(config: &Meta) -> GlobalResult<()> {
	fs::create_dir_all(paths::user_config_dir()?).await?;
	fs::write(paths::meta_config_file()?, serde_json::to_string(config)?).await?;

	Ok(())
}

/// Reads from the global config.
pub async fn try_read_global<F: FnOnce(&Meta) -> GlobalResult<T>, T>(cb: F) -> GlobalResult<T> {
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
		if let Some(project_config) = config.projects.get(&project_root) {
			cb(project_config)
		} else {
			bail!("project not initiated")
		}
	})
	.await
}

/// Non-failable version of `try_read_project`.
pub async fn read_project<F: FnOnce(&ProjectMeta) -> T, T>(cb: F) -> GlobalResult<T> {
	try_read_project(|x| Ok(cb(x))).await
}

pub async fn try_mutate_global<F: FnOnce(&mut Meta) -> GlobalResult<T>, T>(
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
		let project_config = unwrap!(
			config.projects.get_mut(&project_root),
			"project meta does not exist"
		);
		cb(project_config)
	})
	.await
}

/// Non-failable version of `try_mutate_project`.
pub async fn mutate_project<F: FnOnce(&mut ProjectMeta) -> T, T>(cb: F) -> GlobalResult<T> {
	try_mutate_project(|x| Ok(cb(x))).await
}

pub async fn has_project() -> GlobalResult<bool> {
	let project_root = paths::project_root()?;
	let has_project = try_read_global(|meta| Ok(meta.projects.contains_key(&project_root))).await?;
	Ok(has_project)
}

pub async fn insert_project(api_endpoint: String, cloud_token: String) -> GlobalResult<()> {
	let project_root = paths::project_root()?;
	try_mutate_global(|meta| {
		Ok(meta
			.projects
			.insert(project_root, ProjectMeta::new(api_endpoint, cloud_token)))
	})
	.await?;
	Ok(())
}

pub async fn delete_project() -> GlobalResult<()> {
	let project_root = paths::project_root()?;
	try_mutate_global(|x| {
		x.projects.remove(&project_root);
		Ok(())
	})
	.await
}
