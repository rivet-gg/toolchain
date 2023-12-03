use anyhow::{Context, Result};
use clap::Parser;
use cli_core::rivet_api;
use serde::Serialize;

use crate::util::{struct_fmt, term};

#[derive(Parser)]
pub enum SubCommand {
	/// Get the current game
	Get {
		#[clap(long, value_parser)]
		format: struct_fmt::Format,
	},
	/// Show the current game dashboard
	#[clap(alias = "dash")]
	Dashboard,
}

impl SubCommand {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> Result<()> {
		match self {
			SubCommand::Get { format } => {
				let game_res =
					rivet_api::apis::cloud_games_games_api::cloud_games_games_get_game_by_id(
						&ctx.openapi_config_cloud,
						&ctx.game_id,
						None,
					)
					.await
					.context("cloud_games_games_get_game_by_id")?;
				let game_id = game_res.game.game_id.to_string();

				#[derive(Serialize)]
				struct Output<'a> {
					game_id: &'a str,
				}
				struct_fmt::print(format, &Output { game_id: &game_id })?;

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
