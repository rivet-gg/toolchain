use anyhow::{Context, Result};
use clap::Parser;
use serde::Serialize;

use crate::util::{struct_fmt, term};

#[derive(Parser)]
pub enum SubCommand {
	Get {
		#[clap(long, value_parser)]
		format: struct_fmt::Format,
	},
	#[clap(alias = "dash")]
	Dashboard,
}

impl SubCommand {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> Result<()> {
		match self {
			SubCommand::Get { format } => {
				let game_res = ctx
					.client()
					.get_game_by_id()
					.game_id(&ctx.game_id)
					.send()
					.await
					.context("client.get_game_by_id")?;
				let game = game_res.game.context("game_res.game")?;
				let game_id = game.game_id().context("game.game_id")?;

				#[derive(Serialize)]
				struct Output<'a> {
					game_id: &'a str,
				}
				struct_fmt::print(format, &Output { game_id })?;

				Ok(())
			}
			SubCommand::Dashboard => {
				eprintln!("{}", term::link(dashboard_url(&ctx.game_id)));

				Ok(())
			}
		}
	}
}

pub fn dashboard_url(game_id: &str) -> String {
	format!("https://hub.rivet.gg/developer/games/{game_id}")
}
