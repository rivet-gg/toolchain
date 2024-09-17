use anyhow::*;
use sha1::{Digest, Sha1};
use std::{env, path::PathBuf};

/// Root of the current project.
pub fn project_root() -> Result<PathBuf> {
	Ok(env::current_dir()?)
}

/// Returns a unique hash to the current project's path.
pub fn project_path_hash() -> Result<String> {
	let project_root = project_root()?;
	let mut hasher = Sha1::new();
	hasher.update(project_root.to_string_lossy().as_bytes());
	Ok(format!("{:x}", hasher.finalize()))
}

/// Where all data gets stored globally.
pub fn data_dir() -> Result<PathBuf> {
	Ok(dirs::data_dir().context("dirs::data_dir()")?.join("rivet"))
}

/// Global config data.
pub fn user_config_dir(base_data_dir: &PathBuf) -> Result<PathBuf> {
	Ok(base_data_dir.join("config"))
}

/// Global user config file.
pub fn user_settings_config_file(base_data_dir: &PathBuf) -> Result<PathBuf> {
	Ok(user_config_dir(base_data_dir)?.join("settings.json"))
}

/// Project user config file.
pub fn project_settings_config_file() -> Result<PathBuf> {
	Ok(project_root()?.join(".rivet").join("settings.json"))
}

/// Directory specific to this project.
///
/// This is not stored within the project itself since it causes problems with version control &
/// bugs in WSL.
pub fn project_data_dir(base_data_dir: &PathBuf) -> Result<PathBuf> {
	Ok(base_data_dir.join("projects").join(project_path_hash()?))
}

/// Stores all meta.
pub fn meta_config_file(base_data_dir: &PathBuf) -> Result<PathBuf> {
	Ok(project_data_dir(base_data_dir)?.join("meta.json"))
}
