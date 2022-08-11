use anyhow::Result;
use std::{env, path::PathBuf};

pub fn internal_config() -> Result<PathBuf> {
	Ok(env::current_dir()?.join(".rivet"))
}

pub fn cloud_token() -> Result<PathBuf> {
	Ok(internal_config()?.join("cloud_token"))
}
