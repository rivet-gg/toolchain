use anyhow::Result;
use tokio::fs;

use super::paths;

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
