use clap::Parser;
use cli_core::rivet_api::{self};

use global_error::prelude::*;
use serde::Serialize;

use url::Url;

use super::SideKickHandler;

#[derive(Parser)]
pub struct Opts {
	/// The namespace to get the version for
	#[structopt(short, long)]
	namespace: String,
}

#[derive(Serialize)]
pub struct Output {
	pub output: String,
}

impl SideKickHandler for Output {}

impl Opts {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> GlobalResult<Output> {
		// Get the game ID
		let game_res = unwrap!(
			rivet_api::apis::cloud_games_api::cloud_games_get_game_by_id(
				&ctx.openapi_config_cloud,
				&ctx.game_id,
				None,
			)
			.await
		);
		let game_id = game_res.game.game_id.to_string();

		// Build the URL from the game ID and the namespace
		let url = format!(
			"{}/games/{}/namespaces/{}/versions",
			ctx.api_endpoint, game_id, self.namespace
		);

		// Parse the URL and change the subdomain from `api` to `hub`
		let mut parsed_url = Url::parse(&url).unwrap();
		let host = parsed_url.host_str().unwrap().replace("api", "hub");
		parsed_url.set_host(Some(&host)).unwrap();

		Ok(Output {
			output: parsed_url.to_string(),
		})
	}
}
