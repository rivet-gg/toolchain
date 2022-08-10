use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tokio::fs;

use crate::error::Error;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Config {
	pub auth: AuthConfig,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
	pub token: Option<String>,
}

pub fn get_config_path() -> Result<PathBuf, Error> {
	let config_path = home::home_dir()
		.ok_or(Error::CouldNotFindHomeDir)?
		.join(".config")
		.join("rivetctl.json");
	Ok(config_path)
}

pub async fn read() -> Result<Config, Error> {
	let config_path = get_config_path()?;
	let config = match fs::read(&config_path).await {
		Result::Ok(buf) => serde_json::from_slice::<Config>(buf.as_slice())
			.map_err(|source| Error::InvalidGlobalConfig { source })?,
		Result::Err(_) => Config::default(),
	};

	Ok(config)
}

/// Writes a modified config to the file system.
pub async fn write(config: &Config, path: &Path) -> Result<(), Error> {
	// Create parent directory
	let config_parent = path.parent().ok_or_else(|| Error::Internal {
		message: "missing config path parent".into(),
	})?;
	fs::create_dir_all(&config_parent).await?;

	// Write config
	let config_str = serde_json::to_string(config).map_err(|_| Error::Internal {
		message: "failed to encode config".into(),
	})?;
	fs::write(path, config_str).await?;

	Ok(())
}
