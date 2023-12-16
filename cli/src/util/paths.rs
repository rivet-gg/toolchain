use global_error::prelude::*;
use std::{env, path::PathBuf};

pub fn project_root() -> GlobalResult<PathBuf> {
	Ok(env::current_dir()?)
}

pub fn global_config_dir() -> GlobalResult<PathBuf> {
	Ok(unwrap!(dirs::config_dir()).join("rivet"))
}

pub fn global_config_file() -> GlobalResult<PathBuf> {
	Ok(global_config_dir()?.join("config.yaml"))
}
