use anyhow::{Context, Error, Result};
use clap::Parser;
use cli_core::rivet_api::models::CloudVersionConfig;
use serde::Serialize;
use serde_json::json;
use tabled::Tabled;
use tokio::process::Command;
use uuid::Uuid;

use crate::{
	commands::{build, site},
	util::{fmt, struct_fmt, term},
};

#[derive(Parser)]
pub enum SubCommand {
	/// List all versions
	List,

	/// Get details about a version
	Get {
		/// The version ID
		version: String,
	},

	/// Create a new version
	Create {
		/// Name to display for the version
		#[clap(long = "name", alias = "display-name")]
		display_name: String,

		/// Override specific properties of the config
		#[clap(long = "override", short)]
		overrides: Vec<String>,

		/// Namespace to deploy to
		#[clap(short = 'n', long, alias = "ns")]
		namespace: Option<String>,

		#[clap(long, value_parser)]
		format: Option<struct_fmt::Format>,
	},

	/// Pushes the build and site and creates a new version
	PushAndCreate {
		/// Name of the version to create
		#[clap(long = "name", alias = "display-name")]
		display_name: String,

		/// Override specific properties of the config
		#[clap(long = "override", short)]
		overrides: Vec<String>,

		/// Namespace ID to deploy to
		#[clap(short = 'n', long)]
		namespace: Option<String>,

		/// The build tag to upload
		#[clap(long)]
		build_tag: Option<String>,

		/// The name to assign to the build
		#[clap(long)]
		build_name: Option<String>,

		/// The path to the site directory to upload
		#[clap(long)]
		site_path: Option<String>,

		/// The name of the site that will be created
		#[clap(long)]
		site_name: Option<String>,

		#[clap(long, value_parser)]
		format: Option<struct_fmt::Format>,
	},

	/// Returns the config for a version
	ReadConfig {
		/// Override specific properties of the config
		#[clap(long = "override", short)]
		overrides: Vec<String>,

		/// The namespace ID to deploy to
		#[clap(short = 'n', long)]
		namespace: Option<String>,
	},

	/// Show the a version's dashboard
	#[clap(alias("dash"))]
	Dashboard { version: String },
}

impl SubCommand {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> Result<()> {
		match self {
			SubCommand::List => {
				let game_res = ctx
					.client()
					.get_game_by_id()
					.game_id(&ctx.game_id)
					.send()
					.await
					.context("client.get_game_by_id")?;
				let game = game_res.game.context("game_res.game")?;
				let namespaces = game.namespaces().context("game.namespaces")?;

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
					.versions()
					.context("game.versions")?
					.iter()
					.map(|version| {
						let ns = namespaces
							.iter()
							.filter(|ns| ns.version_id() == version.version_id())
							.filter_map(|ns| ns.display_name())
							.collect::<Vec<&str>>()
							.join(", ");

						Ok(Version {
							display_name: version
								.display_name()
								.context("version.display_name")?
								.to_string(),
							namespaces: ns,
							created: fmt::date(version.create_ts().context("version.create_ts")?),
							version_id: version
								.version_id()
								.context("version.version_id")?
								.to_string(),
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
			SubCommand::Create {
				display_name,
				overrides,
				namespace,
				format,
			} => {
				let overrides = parse_config_override_args(overrides)?;
				let output = create(
					ctx,
					display_name,
					overrides,
					namespace.as_ref().map(String::as_str),
				)
				.await?;
				struct_fmt::print_opt(format.as_ref(), &output)?;

				Ok(())
			}
			SubCommand::PushAndCreate {
				display_name,
				overrides,
				namespace,
				build_tag,
				build_name,
				site_path,
				site_name,
				format,
			} => {
				let site_output = if let Some(site_path) = site_path {
					Some(
						site::push(
							ctx,
							&site::SitePushOpts {
								path: site_path.clone(),
								name: site_name.clone(),
								format: format.clone(),
							},
						)
						.await?,
					)
				} else {
					None
				};

				let build_output = if let Some(build_tag) = build_tag {
					Some(
						build::push(
							ctx,
							&build::BuildPushOpts {
								tag: build_tag.clone(),
								name: build_name.clone(),
								format: format.clone(),
							},
						)
						.await?,
					)
				} else {
					None
				};

				// Parse overrides
				let mut overrides = parse_config_override_args(overrides)?;
				if let Some(site_output) = site_output {
					overrides.push(("cdn.site".into(), json!(site_output.site_id)));
				}
				if let Some(build_output) = build_output {
					overrides.push((
						"matchmaker.docker.build".into(),
						json!(build_output.build_id),
					));
				}

				let output = create(
					ctx,
					display_name,
					overrides,
					namespace.as_ref().map(String::as_str),
				)
				.await?;
				struct_fmt::print_opt(format.as_ref(), &output)?;

				Ok(())
			}
			SubCommand::ReadConfig {
				overrides,
				namespace,
			} => {
				let overrides = parse_config_override_args(overrides)?;
				let config = read_config(overrides, namespace.as_ref().map(String::as_str)).await?;
				println!("{:#?}", config);

				Ok(())
			}
			SubCommand::Dashboard { version } => {
				// Check the version exists
				ctx.client()
					.get_game_version_by_id()
					.game_id(&ctx.game_id)
					.version_id(version)
					.send()
					.await
					.context("client.get_game_version_by_id")?;

				eprintln!("{}", term::link(dashboard_url(&ctx.game_id, version)));

				Ok(())
			}
		}
	}
}

async fn print_version(ctx: &cli_core::Ctx, version_id: &str) -> Result<()> {
	let version_res = ctx
		.client()
		.get_game_version_by_id()
		.game_id(&ctx.game_id)
		.version_id(version_id)
		.send()
		.await
		.context("client.get_game_version_by_id")?;
	let version = version_res.version().context("version_res.version")?;

	println!("{version:#?}");

	Ok(())
}

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

pub async fn read_config(
	overrides: Vec<(String, serde_json::Value)>,
	namespace: Option<&str>,
) -> Result<CloudVersionConfig> {
	// Build base config
	let mut config_builder = config::ConfigBuilder::<config::builder::AsyncState>::default()
		.add_source(config::File::with_name("rivet.version"));

	if let Some(namespace) = namespace {
		config_builder = config_builder.add_source(
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
	let config = config_builder
		.build()
		.await
		.context("find version config")?;
	let version = config
		.try_deserialize::<CloudVersionConfig>()
		.context("deserialize version config")?;

	Ok(version)
}

pub async fn process_rivet_config(
	ctx: &cli_core::Ctx,
	version: CloudVersionConfig,
) -> Result<CloudVersionConfig> {
	// TODO: Do this for all possible docker endpoints

	if let Some(docker) = version.matchmaker.as_ref().and_then(|x| x.docker.as_ref()) {
		// Build Docker
		if docker.image_id.is_none() {
			if let Some(dockerfile) = docker.dockerfile.as_ref() {
				// Build image
				let tag = format!("rivet-game:{}", Uuid::new_v4());
				let mut build_cmd = Command::new("docker");
				build_cmd
					.arg("build")
					.arg("--file")
					.arg(dockerfile)
					.arg("--tag")
					.arg(tag)
					.arg(".");
				let build_status = build_cmd.status().await?;
				// TODO: Check status

				// TODO: Upload build
			}
		}
	}

	// Build CDN
	let build_command: Option<String> = Some("build-cdn.sh".into());
	let build_output: Option<String> = Some("dist/".into());
	let site_id: Option<String> = None;
	if site_id.is_none() {
		if let Some(build_output) = build_output {
			if let Some(build_command) = build_command {
				let mut build_cmd = Command::new("/bin/sh");
				build_cmd.arg("-c").arg(build_command);
				build_cmd.status().await?;
				// TODO: Check Windows support
				// TODO: Check status
			}

			// TODO: Upload path
		}
	}

	Ok(version)
}

pub fn dashboard_url(game_id: &str, version_id: &str) -> String {
	format!(
		"https://hub.rivet.gg/developer/games/{game_id}/versions/{version_id}",
		game_id = game_id,
		version_id = version_id
	)
}

#[derive(Serialize)]
pub struct CreateOutput {
	pub version_id: String,
}

pub async fn create(
	ctx: &cli_core::Ctx,
	display_name: &str,
	overrides: Vec<(String, serde_json::Value)>,
	namespace: Option<&str>,
) -> Result<CreateOutput> {
	// Parse config
	let user_config = read_config(overrides, namespace).await?;
	let rivet_config = process_rivet_config(ctx, user_config).await?;

	// Create game version
	let version_res =
		cli_core::rivet_api::apis::cloud_games_versions_api::cloud_games_versions_create_game_version(
			&ctx.openapi_config_cloud,
			&ctx.game_id,
			cli_core::rivet_api::models::CloudGamesCreateGameVersionInput {
				display_name: display_name.into(),
				config: Box::new(rivet_config),
			},
		)
		.await
		.context("versions_create_game_version")?;
	let version_id = version_res.version_id;

	term::status::success("Published", display_name);
	term::status::info("Dashboard", dashboard_url(&ctx.game_id, &version_id));

	Ok(CreateOutput { version_id })
}
