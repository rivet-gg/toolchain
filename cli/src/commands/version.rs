use anyhow::{Context, Error, Result};
use clap::Parser;
use serde::Serialize;
use tabled::Tabled;

use crate::util::{fmt, struct_fmt, term};

#[derive(Parser)]
pub enum SubCommand {
	List,

	Get {
		version: String,
	},

	Create {
		#[clap(long)]
		display_name: String,

		#[clap(long = "override", short)]
		overrides: Vec<String>,

		#[clap(long, value_parser)]
		format: Option<struct_fmt::Format>,
	},

	ReadConfig {
		#[clap(long = "override", short)]
		overrides: Vec<String>,
	},

	#[clap(alias("dash"))]
	Dashboard {
		version: String,
	},
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
				format,
			} => {
				// Parse config
				let overrides = parse_config_override_args(overrides)?;
				let user_config = read_user_config(overrides).await?;
				let rivet_config = build_rivet_config(ctx, &user_config).await?;

				// Create version
				let version_res = ctx
					.client()
					.create_game_version()
					.game_id(&ctx.game_id)
					.display_name(display_name)
					.config(rivet_config)
					.send()
					.await
					.context("client.create_game_version")?;
				let version_id = version_res.version_id().context("version_res.version_id")?;

				term::status::success("Published", display_name);
				term::status::info("Dashboard", dashboard_url(&ctx.game_id, version_id));

				#[derive(Serialize)]
				struct Output<'a> {
					version_id: &'a str,
				}
				struct_fmt::print_opt(format, &Output { version_id })?;

				Ok(())
			}
			SubCommand::ReadConfig { overrides } => {
				let overrides = parse_config_override_args(overrides)?;
				let user_config = read_user_config(overrides).await?;
				println!("=== User Config ===");
				println!("{:#?}", user_config);

				let rivet_config = build_rivet_config(ctx, &user_config).await?;
				println!("=== Rivet Config ===");
				println!("{:#?}", rivet_config);

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

				term::link(dashboard_url(&ctx.game_id, version));

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

pub async fn read_user_config(
	overrides: Vec<(String, serde_json::Value)>,
) -> Result<cli_core::config::version::Version> {
	// Build base config
	let mut config_builder = config::ConfigBuilder::<config::builder::AsyncState>::default()
		.add_source(config::File::with_name("rivet.version"));

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
		.try_deserialize::<cli_core::config::version::Version>()
		.context("deserialize version config")?;

	Ok(version)
}

pub async fn build_rivet_config(
	ctx: &cli_core::Ctx,
	version: &cli_core::config::version::Version,
) -> Result<cli_core::rivet_cloud::model::CloudVersionConfig> {
	// Fetch game
	let game_res = ctx
		.client()
		.get_game_by_id()
		.game_id(&ctx.game_id)
		.send()
		.await
		.context("client.get_game_by_id")?;
	let game = game_res.game().context("game_res.game")?;

	// Build model
	let model = version.build_model(game)?;

	Ok(model)
}

pub fn dashboard_url(game_id: &str, version_id: &str) -> String {
	format!(
		"https://rivet.gg/developer/games/{game_id}/versions/{version_id}",
		game_id = game_id,
		version_id = version_id
	)
}
