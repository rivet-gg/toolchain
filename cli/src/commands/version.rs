use std::collections::HashSet;

use anyhow::{bail, Context, Error, Result};
use clap::Parser;
use cli_core::rivet_api::{apis, models};
use serde::Deserialize;
use tabled::Tabled;
use uuid::Uuid;

use crate::{commands::deploy, util::term};

#[derive(Parser)]
pub enum SubCommand {
	/// List all versions
	List,

	/// Get details about a version
	Get {
		/// The version ID
		version: String,
	},

	/// Returns the config for a version
	#[clap(alias = "read-config")]
	ValidateConfig {
		/// Override specific properties of the config
		#[clap(long = "override", short)]
		overrides: Vec<String>,

		/// The namespace ID to deploy to
		#[clap(short = 'n', long)]
		namespace: Option<String>,
	},

	/// Show the a version's dashboard
	#[clap(alias = "dashboard", alias("dash"))]
	View { version: String },

	/// Deprecated. Use `rivet deploy`.
	///
	/// Pushes the build and site and creates a new version
	#[clap(
		hide = true,
		alias = "push-and-create",
		alias = "create",
		alias = "publish"
	)]
	Deploy(deploy::Opts),
}

impl SubCommand {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> Result<()> {
		match self {
			SubCommand::List => {
				let game_res = apis::cloud_games_games_api::cloud_games_games_get_game_by_id(
					&ctx.openapi_config_cloud,
					&ctx.game_id,
					None,
				)
				.await
				.context("cloud_games_games_get_game_by_id")?;
				let game = &game_res.game;
				let namespaces = &game.namespaces;

				#[derive(Tabled)]
				struct Version {
					#[tabled(rename = "Name")]
					display_name: String,
					#[tabled(rename = "Namespaces")]
					namespaces: String,
					#[tabled(rename = "Created")]
					created: String,
					#[tabled(rename = "ID")]
					version_id: String,
				}

				let mut version = game
					.versions
					.iter()
					.map(|version| {
						let ns = namespaces
							.iter()
							.filter(|ns| ns.version_id == version.version_id)
							.map(|ns| ns.display_name.as_str())
							.collect::<Vec<_>>()
							.join(", ");

						Ok(Version {
							display_name: version.display_name.clone(),
							namespaces: ns,
							created: version.create_ts.clone(),
							version_id: version.version_id.to_string(),
						})
					})
					.collect::<Result<Vec<_>>>()?;
				version.reverse();
				term::table(&version);

				Ok(())
			}
			SubCommand::Get { version } => {
				print_version(ctx, &version).await?;

				Ok(())
			}
			SubCommand::Deploy(opts) => opts.execute(ctx).await,
			SubCommand::ValidateConfig {
				overrides,
				namespace,
			} => {
				let overrides = parse_config_override_args(overrides)?;
				let mut rivet_config =
					read_config(overrides, namespace.as_ref().map(String::as_str)).await?;
				eprintln!("{:#?}", rivet_config);
				build_mock_config_dependencies(&mut rivet_config)?;

				// Validate game version
				let validate_res =
					apis::cloud_games_versions_api::cloud_games_versions_validate_game_version(
						&ctx.openapi_config_cloud,
						&ctx.game_id,
						models::CloudGamesValidateGameVersionRequest {
							display_name: "Mock Display Name".into(),
							config: Box::new(rivet_config),
						},
					)
					.await;
				eprintln!();
				if let Err(err) = validate_res.as_ref() {
					eprintln!("Error: {err:?}");
				}
				let validate_res =
					validate_res.context("cloud_games_versions_validate_game_version")?;
				if !validate_res.errors.is_empty() {
					eprintln!("Found errors:");
					for error in validate_res.errors {
						println!("- {error:?}");
					}
				} else {
					eprintln!("Config is valid.");
				}

				Ok(())
			}
			SubCommand::View { version } => {
				// Check the version exists
				apis::cloud_games_versions_api::cloud_games_versions_get_game_version_by_id(
					&ctx.openapi_config_cloud,
					&ctx.game_id,
					&version,
				)
				.await
				.context("cloud_games_versions_get_game_version_by_id")?;

				eprintln!("{}", term::link(dashboard_url(&ctx.game_id, version)));

				Ok(())
			}
		}
	}
}

/// Prints information about a game version
async fn print_version(ctx: &cli_core::Ctx, version_id: &str) -> Result<()> {
	let version_res = apis::cloud_games_versions_api::cloud_games_versions_get_game_version_by_id(
		&ctx.openapi_config_cloud,
		&ctx.game_id,
		&version_id,
	)
	.await
	.context("cloud_games_versions_get_game_version_by_id")?;
	let version = &version_res.version;

	println!("{version:#?}");

	Ok(())
}

/// Parses config parameters passed to override version parameters
pub fn parse_config_override_args(
	overrides: &[String],
) -> Result<Vec<(String, serde_json::Value)>> {
	overrides
		.iter()
		.map(|value| {
			value
				.split_once("=")
				.context("override needs equal")
				.and_then(|(key, value)| {
					let value_json = serde_json::from_str::<serde_json::Value>(value)
						.context("invalid override value json")?;
					Ok((key.to_string(), value_json))
				})
		})
		.collect::<Result<Vec<_>, Error>>()
}

/// Reads the Rivet configuration file and applies overrides. Uses the
/// namespace to read override files.
///
/// For example, in the namespace `foobar`, Rivet would first read
/// `rivet.yaml` then override with properties from
/// `rivet.foobar.yaml`.
pub async fn read_config(
	overrides: Vec<(String, serde_json::Value)>,
	namespace: Option<&str>,
) -> Result<models::CloudVersionConfig> {
	read_config_inner::<models::CloudVersionConfig>(overrides, namespace).await
}

#[derive(Deserialize)]
pub struct CloudVersionConfigPartial {
	#[serde(rename = "engine", skip_serializing_if = "Option::is_none")]
	pub engine: Option<Box<models::CloudVersionEngineConfig>>,
}

/// Similar to `read_config`. Reads a partial config that can be used for `rivet init`.
pub async fn read_config_partial(
	overrides: Vec<(String, serde_json::Value)>,
	namespace: Option<&str>,
) -> Result<CloudVersionConfigPartial> {
	read_config_inner::<CloudVersionConfigPartial>(overrides, namespace).await
}

/// Reads the version config to the given type. See `read_config` and `read_config_partial`.
async fn read_config_inner<T: serde::de::DeserializeOwned>(
	overrides: Vec<(String, serde_json::Value)>,
	namespace: Option<&str>,
) -> Result<T> {
	// Check for conflicting .yaml and .yml file suffixes
	//
	// It's almost always a mistake when this happens, so we fail by default here.
	let mut files = HashSet::new();
	let mut dir = tokio::fs::read_dir(".").await?;
	while let Some(entry) = dir.next_entry().await? {
		let path = entry.path();
		if let Some(ext) = path.extension() {
			if ext == "yaml" || ext == "yml" {
				let Some(file_name) = path
					.file_stem()
					.and_then(|x| x.to_str())
					.map(|x| x.to_owned())
				else {
					continue;
				};

				if file_name.starts_with("rivet") {
					if files.contains(&file_name) {
						bail!("Found conflicting config files: {0}.yaml and {0}.yml. Please remove one.", file_name);
					}

					files.insert(file_name);
				}
			}
		}
	}

	// Build base config
	let mut config_builder = config::ConfigBuilder::<config::builder::AsyncState>::default()
		.add_source(config::File::with_name("rivet").required(false))
		// Support legacy `rivet.version.toml` file name
		.add_source(config::File::with_name("rivet.version").required(false));

	if let Some(namespace) = namespace {
		config_builder = config_builder
			.add_source(config::File::with_name(&format!("rivet.{namespace}")).required(false))
			.add_source(
				config::File::with_name(&format!("rivet.version.{namespace}")).required(false),
			);
	}

	// Apply overrides
	for (k, v) in overrides {
		#[derive(serde::Serialize)]
		struct Empty {
			root: serde_json::Value,
		}

		// Parse the JSON data to a config Value that we can pass as an
		// override.
		//
		// We have to embed the value in `Empty` because the value can't be at
		// the root of the config.
		let config_value = config::Config::try_from(&Empty { root: v })
			.context("read override value to config value")?
			.get::<config::Value>("root")?;

		// Add the override
		config_builder = config_builder
			.set_override(k, config_value)
			.context("set override")?;
	}

	// Read config
	let config = config_builder.build().await.context("find config")?;
	let version = config
		.try_deserialize::<T>()
		.context("deserialize config")?;

	Ok(version)
}

/// Fill in dummy information for fields that will be eventually filled in by the
/// build config dependencies.
///
/// Used to build a config file that wil pass validation.
pub fn build_mock_config_dependencies(version: &mut models::CloudVersionConfig) -> Result<()> {
	if let Some(matchmaker) = version.matchmaker.as_mut() {
		if let Some(docker) = matchmaker.docker.as_mut() {
			docker.image_id = Some(Uuid::nil());
		}

		if let Some(game_modes) = matchmaker.game_modes.as_mut() {
			for (_, game_mode) in game_modes.iter_mut() {
				if let Some(docker) = game_mode.docker.as_mut() {
					docker.image_id = Some(Uuid::nil());
				}
			}
		}
	}

	// Build CDN
	if let Some(cdn) = version.cdn.as_mut() {
		cdn.site_id = Some(Uuid::nil());
	}

	Ok(())
}

pub fn dashboard_url(game_id: &str, version_id: &str) -> String {
	format!(
		"https://hub.rivet.gg/developer/games/{game_id}/versions/{version_id}",
		game_id = game_id,
		version_id = version_id
	)
}

pub fn rivet_game_url(game_name_id: &str, namespace_name_id: &str) -> String {
	if namespace_name_id == "prod" {
		format!("https://{game_name_id}.rivet.game/")
	} else {
		format!("https://{game_name_id}--{namespace_name_id}.rivet.game/")
	}
}
