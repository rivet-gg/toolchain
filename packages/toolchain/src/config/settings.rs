use anyhow::*;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::sync::Mutex;

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
	pub source_path: Option<String>,

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
		"rivet.dev.json".to_string()
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
		"rivet.json".into()
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

lazy_static! {
	static ref GLOBAL_SETTINGS: Mutex<HashMap<PathBuf, Settings>> = Mutex::new(HashMap::new());
}

async fn get_or_init<F, T>(data_dir: &PathBuf, cb: F) -> Result<T>
where
	F: FnOnce(&mut Settings) -> Result<T>,
{
	let mut global_settings = GLOBAL_SETTINGS.lock().await;

	if !global_settings.contains_key(data_dir) {
		let mut config_builder = config::ConfigBuilder::<config::builder::AsyncState>::default();

		if paths::user_settings_config_file(data_dir)?.exists() {
			config_builder = config_builder.add_source(config::File::from(
				paths::user_settings_config_file(data_dir)?,
			));
		}
		if paths::project_settings_config_file()?.exists() {
			config_builder = config_builder
				.add_source(config::File::from(paths::project_settings_config_file()?));
		}

		let config = config_builder.build().await.context("find config")?;
		let mut settings = config
			.try_deserialize::<Settings>()
			.context("deserialize config")?;

		let result = cb(&mut settings)?;
		global_settings.insert(data_dir.clone(), settings);
		Ok(result)
	} else {
		let settings = global_settings
			.get_mut(data_dir)
			.context("global_settings[data_dir]")?;
		cb(settings)
	}
}

pub async fn try_read<F, T>(data_dir: &PathBuf, cb: F) -> Result<T>
where
	F: FnOnce(&Settings) -> Result<T>,
{
	get_or_init(data_dir, |settings| cb(settings)).await
}

pub async fn try_mutate<F, T>(data_dir: &PathBuf, cb: F) -> Result<T>
where
	F: FnOnce(&mut Settings) -> Result<T>,
{
	get_or_init(data_dir, cb).await
}
