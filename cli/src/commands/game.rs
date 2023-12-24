use clap::Parser;
use cli_core::rivet_api::apis;
use global_error::prelude::*;
use serde::Serialize;

use crate::util::{struct_fmt, term};

#[derive(Parser)]
pub enum SubCommand {
	/// Get the current game
	Get {
		#[clap(long, value_parser)]
		format: struct_fmt::Format,
	},

	/// Open the game in the hub
	#[clap(alias = "dashboard", alias = "dash")]
	View,
}

impl SubCommand {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> GlobalResult<()> {
		match self {
			SubCommand::Get { format } => {
				let game_res = unwrap!(
					apis::cloud_games_games_api::cloud_games_games_get_game_by_id(
						&ctx.openapi_config_cloud,
						&ctx.game_id,
						None,
					)
					.await
				);
				let game_id = game_res.game.game_id.to_string();

				#[derive(Serialize)]
				struct Output<'a> {
					game_id: &'a str,
				}
				struct_fmt::print(format, &Output { game_id: &game_id })?;

				Ok(())
			}
			SubCommand::View => {
				eprintln!("{}", term::link(dashboard_url(&ctx, &ctx.game_id)));

				Ok(())
			}
		}
	}
}

pub fn dashboard_url(ctx: &cli_core::Ctx, game_id: &str) -> String {
	format!("{}/developer/games/{game_id}", ctx.bootstrap.origins.hub)
}
