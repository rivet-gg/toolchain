use anyhow::{Context, Result};
use clap::Parser;
use tabled::{Table, Tabled};

#[derive(Parser)]
pub enum SubCommand {
	List,
	Create {
		display_name: String,
		version: String,
		name_id: String,
	},
	SetVersion,
	Dashboard,
	Visit,
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
				struct Namespace {
					#[tabled(rename = "Name")]
					display_name: String,
					#[tabled(rename = "Name ID")]
					name_id: String,
					#[tabled(rename = "ID")]
					namespace_id: String,
				}

				let ns = game
					.namespaces()
					.context("game.namespaces")?
					.iter()
					.map(|ns| {
						Ok(Namespace {
							display_name: ns.display_name().context("ns.display_name")?.to_string(),
							name_id: ns.name_id().context("ns.name_id")?.to_string(),
							namespace_id: ns.namespace_id().context("ns.namespace_id")?.to_string(),
						})
					})
					.collect::<Result<Vec<_>>>()?;
				println!("{}", Table::new(&ns));

				Ok(())
			}
			SubCommand::Create {
				display_name,
				version,
				name_id,
			} => {
				ctx.client()
					.create_game_namespace()
					.display_name(display_name)
					.name_id(name_id)
					.version_id(version)
					.send()
					.await
					.context("client.create_game_namespace")?;
				Ok(())
			}
			SubCommand::SetVersion => todo!(),
			SubCommand::Dashboard => todo!(),
			SubCommand::Visit => todo!(),
		}
	}
}
