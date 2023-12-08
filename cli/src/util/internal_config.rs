use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::{
	fs,
	sync::{Mutex, OnceCell},
};

use super::paths;

/// Config stored in .rivet/config.yaml. Used to store persistent data.
#[derive(Default, Serialize, Deserialize)]
pub struct InternalConfig {
	#[serde(default)]
	pub cluster: Cluster,
	#[serde(default)]
	pub telemetry: Telemetry,
	#[serde(default)]
	pub tokens: Tokens,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Cluster {
	#[serde(default)]
	pub api_endpoint: Option<String>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Telemetry {
	#[serde(default)]
	pub disabled: bool,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Tokens {
	#[serde(default)]
	pub cloud: Option<String>,
}

static SINGLETON: OnceCell<Mutex<InternalConfig>> = OnceCell::const_new();

async fn get_or_init() -> Result<&'static Mutex<InternalConfig>> {
	SINGLETON
		.get_or_try_init::<anyhow::Error, _, _>(|| async {
			let path = paths::internal_config_file()?;

			let config = match fs::read_to_string(&path).await {
				Ok(config) => serde_yaml::from_str(&config).map_err(Into::<anyhow::Error>::into)?,
				Err(err) if err.kind() == std::io::ErrorKind::NotFound => InternalConfig::default(),
				Err(err) => return Err(err.into()),
			};

			Result::Ok(Mutex::new(config))
		})
		.await
}

pub async fn write(config: &InternalConfig) -> Result<()> {
	fs::create_dir_all(paths::internal_config()?).await?;
	fs::write(
		paths::internal_config_file()?,
		serde_yaml::to_string(config)?,
	)
	.await?;

	Ok(())
}

pub async fn read<F: FnOnce(&InternalConfig) -> T, T>(cb: F) -> Result<T> {
	let singleton = get_or_init().await?;
	let mut lock = singleton.lock().await;

	// Fetch value
	let value = cb(&mut *lock);

	Ok(value)
}

pub async fn mutate<F: FnOnce(&mut InternalConfig) -> ()>(cb: F) -> Result<()> {
	let singleton = get_or_init().await?;
	let mut lock = singleton.lock().await;

	// Mutate the config
	cb(&mut *lock);

	// Write new changes
	write(&*lock).await?;

	Ok(())
}
