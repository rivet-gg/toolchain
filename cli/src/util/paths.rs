use anyhow::{Context, Result};
use std::{env, path::PathBuf};

pub fn project_root() -> Result<PathBuf> {
	Ok(env::current_dir()?)
}

pub fn global_config_dir() -> Result<PathBuf> {
	Ok(dirs::config_dir().context("config dir")?.join("rivet"))
}

pub fn global_config_file() -> Result<PathBuf> {
	Ok(global_config_dir()?.join("config.yaml"))
}
