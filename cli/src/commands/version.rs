use anyhow::{Context, Result};
use clap::Parser;
use tabled::Tabled;

use crate::util::{fmt, term};

#[derive(Parser)]
pub enum SubCommand {
	List,
	Get { version_id: String },
	Create,
	Dashboard,
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
			SubCommand::Get { version_id } => {
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
			SubCommand::Create => {
				todo!()
			}
			SubCommand::Dashboard => {
				todo!()
			}
		}
	}
}
