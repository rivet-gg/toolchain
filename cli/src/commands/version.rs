use anyhow::{Context, Result};
use clap::Parser;
use tabled::Tabled;

use crate::util::{fmt, term};

#[derive(Parser)]
pub enum SubCommand {
	List,
	Get {
		version: String,
	},
	Create,
	ReadConfig,
	#[clap(alias("dash"))]
	Dashboard {
		version: String,
	},
}

impl SubCommand {
	pub async fn execute(&self, ctx: &rivetctl::Ctx) -> Result<()> {
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
			SubCommand::Create => {
				todo!()
			}
			SubCommand::ReadConfig => {
				let version = read_config().await?;
				println!("{:#?}", version);

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

				println!(
					"https://rivet.gg/developer/games/{game_id}/versions/{version_id}",
					game_id = ctx.game_id,
					version_id = version
				);

				Ok(())
			}
		}
	}
}

async fn print_version(ctx: &rivetctl::Ctx, version_id: &str) -> Result<()> {
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

pub async fn read_config() -> Result<rivetctl::config::version::Version> {
	let config = config::ConfigBuilder::<config::builder::AsyncState>::default()
		.add_source(config::File::with_name("rivet.version"))
		.build()
		.await
		.context("find version config")?;
	let version = config
		.try_deserialize::<rivetctl::config::version::Version>()
		.context("deserialize version config")?;

	Ok(version)
}
