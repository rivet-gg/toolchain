use anyhow::*;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};
use tokio::{fs, sync::Mutex};
use uuid::Uuid;

use crate::paths;

/// Configuration stored globally on the file system.
#[derive(Default, Serialize, Deserialize)]
pub struct Meta {
	/// Store project meta by the absolute path to the project.
	pub projects: HashMap<PathBuf, ProjectMeta>,
}

/// Config stored in {data_dir}/meta.json. Used to store persistent data, such as tokens & cache.
#[derive(Serialize, Deserialize)]
pub struct ProjectMeta {
	pub cluster: Cluster,
	pub tokens: Tokens,
	pub environments: HashMap<Uuid, Environment>,

	/// Stores the state for all of the processes.
	///
	/// Key is the key in the `ProcessManager` config.
	#[serde(default)]
	pub processes: HashMap<String, ProcessState>,

	/// Port which the dev server is running on for plugins.
	#[serde(default)]
	pub backend_port: Option<u16>,

	/// See `backend_port`.
	#[serde(default)]
	pub editor_port: Option<u16>,
}

impl ProjectMeta {
	fn new(api_endpoint: String, cloud_token: String) -> Self {
		ProjectMeta {
			cluster: Cluster { api_endpoint },
			tokens: Tokens { cloud: cloud_token },
			environments: HashMap::new(),
			processes: HashMap::new(),
			backend_port: None,
			editor_port: None,
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

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Environment {
	#[serde(default)]
	pub backend: Backend,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Backend {
	#[serde(default)]
	pub db_url: Option<String>,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct ProcessState {
	/// ID of the running process.
	///
	/// This is not the same as the PID.
	pub process_id: Option<Uuid>,
}

lazy_static! {
	static ref GLOBAL_META: Mutex<HashMap<PathBuf, Meta>> = Mutex::new(HashMap::new());

	/// Lock on writing to the file.
	static ref META_FILE_LOCK: Mutex<()> = Mutex::new(());
}

/// Gets the global config instance.
///
/// Use `read` to read properties from the config.
async fn get_or_init<F, T>(base_data_dir: &PathBuf, cb: F) -> Result<T>
where
	F: FnOnce(&mut Meta) -> Result<T>,
{
	let mut global_meta = GLOBAL_META.lock().await;

	if !global_meta.contains_key(base_data_dir) {
		let path = paths::meta_config_file(base_data_dir)?;

		let mut config = match fs::read_to_string(&path).await {
			Result::Ok(config) => serde_json::from_str(&config)
				.context(format!("deserialize meta ({})", path.display()))?,
			Err(err) if err.kind() == std::io::ErrorKind::NotFound => Meta::default(),
			Err(err) => return Err(err.into()),
		};

		let result = cb(&mut config)?;
		global_meta.insert(base_data_dir.clone(), config);
		Ok(result)
	} else {
		let meta = global_meta
			.get_mut(base_data_dir)
			.context("global_meta[base_data_dir]")?;
		cb(meta)
	}
}

/// Writes the config to the file system.
///
/// Use `mutate` to make changes to the config publicly.
async fn write(base_data_dir: &PathBuf) -> Result<()> {
	let json_str = get_or_init(base_data_dir, |meta| {
		serde_json::to_string(meta).map_err(Into::into)
	})
	.await?;

	{
		let _write_guard = META_FILE_LOCK.lock().await;
		fs::create_dir_all(paths::user_config_dir(base_data_dir)?).await?;
		fs::write(paths::meta_config_file(base_data_dir)?, json_str).await?;
	}

	Ok(())
}

/// Reads from the global config.
pub async fn try_read_global<F: FnOnce(&Meta) -> Result<T>, T>(
	base_data_dir: &PathBuf,
	cb: F,
) -> Result<T> {
	get_or_init(base_data_dir, |meta| cb(&*meta)).await
}

/// Reads from the project meta.
///
/// If project meta does not exist, returns the default value.
pub async fn try_read_project<F: FnOnce(&ProjectMeta) -> Result<T>, T>(
	base_data_dir: &PathBuf,
	cb: F,
) -> Result<T> {
	let project_root = paths::project_root()?;
	try_read_global(base_data_dir, |config| {
		if let Some(project_config) = config.projects.get(&project_root) {
			cb(project_config)
		} else {
			bail!("project not initiated")
		}
	})
	.await
}

/// Non-failable version of `try_read_project`.
pub async fn read_project<F: FnOnce(&ProjectMeta) -> T, T>(
	base_data_dir: &PathBuf,
	cb: F,
) -> Result<T> {
	try_read_project(base_data_dir, |x| Ok(cb(x))).await
}

pub async fn try_mutate_global<F: FnOnce(&mut Meta) -> Result<T>, T>(
	base_data_dir: &PathBuf,
	cb: F,
) -> Result<T> {
	let res = get_or_init(base_data_dir, |meta| {
		// Mutate meta
		let res = cb(&mut *meta)?;

		Result::Ok(res)
	})
	.await?;

	// Write new changes
	write(base_data_dir).await?;

	Ok(res)
}

/// Mutates the project meta.
///
/// If the project meta does not exist, a default one will be inserted and modified.
pub async fn try_mutate_project<F: FnOnce(&mut ProjectMeta) -> Result<T>, T>(
	base_data_dir: &PathBuf,
	cb: F,
) -> Result<T> {
	let project_root = paths::project_root()?;
	try_mutate_global(base_data_dir, |config| {
		let project_config = config
			.projects
			.get_mut(&project_root)
			.context("project meta does not exist")?;
		cb(project_config)
	})
	.await
}

/// Non-failable version of `try_mutate_project`.
pub async fn mutate_project<F: FnOnce(&mut ProjectMeta) -> T, T>(
	base_data_dir: &PathBuf,
	cb: F,
) -> Result<T> {
	try_mutate_project(base_data_dir, |x| Ok(cb(x))).await
}

pub async fn has_project(base_data_dir: &PathBuf) -> Result<bool> {
	let project_root = paths::project_root()?;
	let has_project = try_read_global(base_data_dir, |meta| {
		Ok(meta.projects.contains_key(&project_root))
	})
	.await?;
	Ok(has_project)
}

pub async fn insert_project(
	base_data_dir: &PathBuf,
	api_endpoint: String,
	cloud_token: String,
) -> Result<()> {
	let project_root = paths::project_root()?;
	try_mutate_global(base_data_dir, |meta| {
		Ok(meta
			.projects
			.insert(project_root, ProjectMeta::new(api_endpoint, cloud_token)))
	})
	.await?;
	Ok(())
}

pub async fn delete_project(base_data_dir: &PathBuf) -> Result<()> {
	let project_root = paths::project_root()?;
	try_mutate_global(base_data_dir, |x| {
		x.projects.remove(&project_root);
		Ok(())
	})
	.await
}
