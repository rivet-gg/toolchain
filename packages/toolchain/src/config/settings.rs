use anyhow::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::{Mutex, OnceCell};

use crate::{
	paths,
	util::docker::{build::DockerBuildMethod, BuildCompression, BuildKind},
};

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Settings {
	#[serde(default)]
	pub backend: BackendConfig,
	#[serde(default)]
	pub game_server: GameServerConfig,
	#[serde(default)]
	pub net: NetConfig,
}

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BackendConfig {
	/// Env vars to pass to all backend commands.
	#[serde(default)]
	pub command_environment: HashMap<String, String>,
	#[serde(default)]
	pub backend_source_path: Option<String>,

	#[serde(default)]
	pub sdk: BackendSdkConfig,
	#[serde(default)]
	pub dev: BackendDevConfig,
	#[serde(default)]
	pub deploy: BackendDeployConfig,
}

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BackendSdkConfig {
	/// Env vars to pass to the deploy backend commands.
	#[serde(default)]
	pub command_environment: HashMap<String, String>,
	#[serde(default)]
	pub path: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BackendDevConfig {
	/// Env vars to pass to the deploy backend commands.
	#[serde(default)]
	pub command_environment: HashMap<String, String>,
	/// Backend ocnfig to use when running backend config.
	#[serde(default = "BackendDevConfig::default_config_path")]
	pub config_path: String,
}

impl Default for BackendDevConfig {
	fn default() -> Self {
		Self {
			command_environment: HashMap::new(),
			config_path: Self::default_config_path(),
		}
	}
}

impl BackendDevConfig {
	fn default_config_path() -> String {
		"backend.dev.json".to_string()
	}
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BackendDeployConfig {
	/// Env vars to pass to the deploy backend commands.
	#[serde(default)]
	pub command_environment: HashMap<String, String>,
	/// Backend ocnfig to use when running backend config.
	#[serde(default = "BackendDeployConfig::default_config_path")]
	pub config_path: String,
}

impl Default for BackendDeployConfig {
	fn default() -> Self {
		Self {
			command_environment: HashMap::new(),
			config_path: Self::default_config_path(),
		}
	}
}

impl BackendDeployConfig {
	fn default_config_path() -> String {
		"backend.json".into()
	}
}

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameServerConfig {
	#[serde(default)]
	pub deploy: GameServerDeployConfig,
}

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameServerDeployConfig {
	#[serde(default)]
	pub dockerfile_path: Option<String>,
	#[serde(default)]
	pub docker_image: Option<String>,
	#[serde(default)]
	pub build_args: Option<HashMap<String, String>>,
	#[serde(default)]
	pub allow_root: bool,
	#[serde(default)]
	pub build_method: DockerBuildMethod,
	#[serde(default)]
	pub build_kind: BuildKind,
	// Default value depends on build_kind
	#[serde(default)]
	pub build_compression: Option<BuildCompression>,
}

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NetConfig {
	#[serde(default)]
	pub disable_upload_multipart: bool,
}

static SINGLETON: OnceCell<Mutex<Settings>> = OnceCell::const_new();

async fn read() -> Result<&'static Mutex<Settings>> {
	let config = SINGLETON
		.get_or_try_init::<anyhow::Error, _, _>(|| async {
			let mut config_builder =
				config::ConfigBuilder::<config::builder::AsyncState>::default();

			if paths::user_settings_config_file()?.exists() {
				config_builder = config_builder
					.add_source(config::File::from(paths::user_settings_config_file()?));
			}
			if paths::project_settings_config_file()?.exists() {
				config_builder = config_builder
					.add_source(config::File::from(paths::project_settings_config_file()?));
			}

			let config = config_builder.build().await.context("find config")?;
			let config_deserialized = config
				.try_deserialize::<Settings>()
				.context("deserialize config")?;

			Result::Ok(Mutex::new(config_deserialized))
		})
		.await?;
	Ok(config)
}

pub async fn try_read<F: FnOnce(&Settings) -> Result<T>, T>(cb: F) -> Result<T> {
	let singleton = read().await?;
	let mut lock = singleton.lock().await;

	// Fetch value
	let value = cb(&mut *lock)?;

	Ok(value)
}
