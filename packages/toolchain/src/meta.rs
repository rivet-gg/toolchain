use anyhow::*;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};
use tokio::{fs, sync::Mutex};
use uuid::Uuid;

use crate::paths;

/// Config stored in {data_dir}/meta.json. Used to store persistent data, such as tokens & cache.
#[derive(Default, Serialize, Deserialize)]
pub struct Meta {
	/// If signed in to Rivet, this will include relevant information.
	///
	/// If not signed in, will be None.
	pub cloud: Option<Cloud>,
}

#[derive(Serialize, Deserialize)]
pub struct Cloud {
	/// Rivet API endpoint to connect to.
	pub api_endpoint: String,

	/// Cloud token used to authenticate all API requests.
	pub cloud_token: String,

	/// Cache of all environments for this game.
	pub environments: HashMap<Uuid, Environment>,
}

impl Cloud {
	pub fn new(api_endpoint: String, cloud_token: String) -> Self {
		Self {
			api_endpoint,
			cloud_token,
			environments: HashMap::new(),
		}
	}
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Environment {}

lazy_static! {
	/// List of all meta paths cached in memory.
	///
	/// We can't assume the toolchain will only load one meta, so we need to support multiple
	/// metas.
	static ref META: Mutex<HashMap<PathBuf, Meta>> = Mutex::new(HashMap::new());

	/// Lock on writing to the file.
	static ref META_FILE_LOCK: Mutex<()> = Mutex::new(());
}

/// Writes the config to the file system.
///
/// Use `mutate` to make changes to the config publicly.
async fn write(base_data_dir: &PathBuf, meta: &Meta) -> Result<()> {
	// Serialize meta
	let json_str = serde_json::to_string(meta)?;

	// Write file
	let _write_guard = META_FILE_LOCK.lock().await;
	let path = paths::meta_config_file(base_data_dir)?;
	if let Some(parent) = path.parent() {
		fs::create_dir_all(parent).await?;
	}
	fs::write(path, json_str).await?;

	Ok(())
}

/// Reads from the project meta.
///
/// If project meta does not exist, returns the default value.
pub async fn try_read_project<F: FnOnce(&Meta) -> Result<T>, T>(
	base_data_dir: &PathBuf,
	cb: F,
) -> Result<T> {
	let meta_path = paths::meta_config_file(base_data_dir)?;
	let mut global_meta = META.lock().await;
	if !global_meta.contains_key(&meta_path) {
		let mut meta = match fs::read_to_string(&meta_path).await {
			Result::Ok(config) => serde_json::from_str::<Meta>(&config)
				.context(format!("deserialize meta ({})", meta_path.display()))?,
			Err(err) if err.kind() == std::io::ErrorKind::NotFound => Meta::default(),
			Err(err) => return Err(err.into()),
		};

		let result = cb(&mut meta)?;
		global_meta.insert(meta_path.clone(), meta);

		Ok(result)
	} else {
		let meta = global_meta
			.get_mut(&meta_path)
			.context("global_meta[meta_path]")?;
		cb(meta)
	}
}

/// Non-failable version of `try_read_project`.
pub async fn read_project<F: FnOnce(&Meta) -> T, T>(base_data_dir: &PathBuf, cb: F) -> Result<T> {
	try_read_project(base_data_dir, |x| Ok(cb(x))).await
}

/// Mutates the project meta.
///
/// If the project meta does not exist, a default one will be inserted and modified.
pub async fn try_mutate_project<F: FnOnce(&mut Meta) -> Result<T>, T>(
	base_data_dir: &PathBuf,
	cb: F,
) -> Result<T> {
	// Get project
	let meta_path = paths::meta_config_file(base_data_dir)?;
	let mut global_meta = META.lock().await;
	if !global_meta.contains_key(&meta_path) {
		let mut meta = match fs::read_to_string(&meta_path).await {
			Result::Ok(config) => serde_json::from_str::<Meta>(&config)
				.context(format!("deserialize meta ({})", meta_path.display()))?,
			Err(err) if err.kind() == std::io::ErrorKind::NotFound => Meta::default(),
			Err(err) => return Err(err.into()),
		};

		let result = cb(&mut meta)?;
		write(base_data_dir, &meta).await?;

		Ok(result)
	} else {
		let meta = global_meta
			.get_mut(&meta_path)
			.context("global_meta[meta_path]")?;
		let result = cb(meta)?;
		write(base_data_dir, &meta).await?;

		Ok(result)
	}
}

/// Non-failable version of `try_mutate_project`.
pub async fn mutate_project<F: FnOnce(&mut Meta) -> T, T>(
	base_data_dir: &PathBuf,
	cb: F,
) -> Result<T> {
	try_mutate_project(base_data_dir, |x| Ok(cb(x))).await
}
