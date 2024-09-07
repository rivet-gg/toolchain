use anyhow::*;
use std::{env, path::PathBuf};

pub fn project_root() -> Result<PathBuf> {
	Ok(env::current_dir()?)
}

pub fn data_dir() -> Result<PathBuf> {
	Ok(dirs::data_dir().context("dirs::data_dir()")?.join("rivet"))
}

pub fn user_config_dir() -> Result<PathBuf> {
	Ok(dirs::config_dir()
		.context("dirs::config_dir()")?
		.join("rivet"))
}

pub fn meta_config_file() -> Result<PathBuf> {
	Ok(user_config_dir()?.join("meta.json"))
}

pub fn user_settings_config_file() -> Result<PathBuf> {
	Ok(user_config_dir()?.join("settings.json"))
}

pub fn project_settings_config_file() -> Result<PathBuf> {
	Ok(project_root()?.join(".rivet").join("settings.json"))
}
