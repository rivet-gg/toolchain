use anyhow::{Context, Result};
use clap::Parser;
use tabled::Tabled;

use crate::util::term;

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

				#[derive(Tabled)]
				struct Version {
					#[tabled(rename = "Name")]
					display_name: String,
					#[tabled(rename = "ID")]
					version_id: String,
				}

				let ns = game
					.versions()
					.context("game.versions")?
					.iter()
					.map(|ns| {
						Ok(Version {
							display_name: ns.display_name().context("ns.display_name")?.to_string(),
							version_id: ns.version_id().context("ns.version_id")?.to_string(),
						})
					})
					.collect::<Result<Vec<_>>>()?;
				term::table(&ns);

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
