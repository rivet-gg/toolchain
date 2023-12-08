use anyhow::Result;
use std::{env, path::PathBuf};

pub fn internal_config() -> Result<PathBuf> {
	Ok(env::current_dir()?.join(".rivet"))
}

pub fn internal_config_file() -> Result<PathBuf> {
	Ok(internal_config()?.join("config.yaml"))
}
