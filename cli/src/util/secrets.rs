use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::fs;

use super::paths;

#[derive(Default, Serialize, Deserialize)]
pub struct InternalConfig {
	#[serde(default)]
	pub api_endpoint: Option<String>,
	#[serde(default)]
	pub telemetry_disabled: bool,
}

impl InternalConfig {
	pub async fn read() -> Result<Self> {
		let path = paths::internal_config_file()?;

		match fs::read_to_string(&path).await {
			Ok(config) => toml::from_str(&config).map_err(Into::into),
			Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
				fs::create_dir_all(paths::internal_config()?).await?;
				fs::write(&path, "").await?;

				Ok(InternalConfig::default())
			}
			Err(err) => Err(err.into()),
		}
	}

	pub async fn write(&self) -> Result<()> {
		fs::create_dir_all(paths::internal_config()?).await?;
		fs::write(paths::internal_config_file()?, toml::to_string(&self)?).await?;

		Ok(())
	}
}

pub async fn read_token() -> Result<Option<String>> {
	match fs::read_to_string(paths::cloud_token()?).await {
		Ok(token) => Ok(Some(token)),
		Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
		Err(err) => Err(err.into()),
	}
}

pub async fn write_cloud_token(token: &str) -> Result<()> {
	fs::create_dir_all(paths::internal_config()?).await?;
	fs::write(paths::cloud_token()?, token).await?;
	Ok(())
}
