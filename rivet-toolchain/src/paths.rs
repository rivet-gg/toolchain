use global_error::prelude::*;
use std::{env, path::PathBuf};

pub fn project_root() -> GlobalResult<PathBuf> {
	Ok(env::current_dir()?)
}

pub fn data_dir() -> GlobalResult<PathBuf> {
	Ok(unwrap!(dirs::data_dir()).join("rivet"))
}

pub fn user_config_dir() -> GlobalResult<PathBuf> {
	Ok(unwrap!(dirs::config_dir()).join("rivet"))
}

pub fn meta_config_file() -> GlobalResult<PathBuf> {
	Ok(user_config_dir()?.join("meta.json"))
}

pub fn user_settings_config_file() -> GlobalResult<PathBuf> {
	Ok(user_config_dir()?.join("settings.json"))
}

pub fn project_settings_config_file() -> GlobalResult<PathBuf> {
	Ok(project_root()?.join(".rivet").join("settings.json"))
}
