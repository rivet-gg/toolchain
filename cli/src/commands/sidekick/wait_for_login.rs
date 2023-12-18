use clap::Parser;
use cli_core::{ctx, rivet_api::apis};

use global_error::prelude::*;
use serde::Serialize;

use crate::util::global_config;

use super::SideKickHandler;

#[derive(Parser)]
pub struct Opts {
	/// The token to poll for
	#[structopt(long)]
	device_link_token: String,
}

#[derive(Serialize)]
pub struct Output {
	pub output: String,
}

impl SideKickHandler for Output {}

impl Opts {
	pub async fn execute(&self) -> GlobalResult<Output> {
		let (api_endpoint, _token) = global_config::read_project(|x| {
			(x.cluster.api_endpoint.clone(), x.tokens.cloud.clone())
		})
		.await?;

		// Create OpenAPI configuration without bearer token to send link request
		let openapi_config_cloud_unauthed = apis::configuration::Configuration {
			base_path: api_endpoint
				.clone()
				.unwrap_or_else(|| ctx::DEFAULT_API_ENDPOINT.to_string()),
			user_agent: Some(ctx::user_agent()),
			..Default::default()
		};

		// Wait for link to complete
		let mut watch_index = None;
		let token = loop {
			let prepare_res = unwrap!(
				apis::cloud_devices_links_api::cloud_devices_links_get(
					&openapi_config_cloud_unauthed,
					&self.device_link_token,
					watch_index.as_ref().map(String::as_str),
				)
				.await
			);

			watch_index = Some(prepare_res.watch.index);

			if let Some(token) = prepare_res.cloud_token {
				break token;
			}
		};

		// Create new context
		let new_ctx = cli_core::ctx::init(
			api_endpoint,
			// Exclude overridden access token to check the token
			token.clone(),
		)
		.await?;

		// Inspect the token
		let inspect_res =
			unwrap!(apis::cloud_auth_api::cloud_auth_inspect(&new_ctx.openapi_config_cloud).await);

		// Find the game ID
		let game_id = unwrap!(inspect_res.agent.game_cloud).game_id;

		// Extract game data
		let _game_res = unwrap!(
			apis::cloud_games_games_api::cloud_games_games_get_game_by_id(
				&new_ctx.openapi_config_cloud,
				&game_id.to_string(),
				None,
			)
			.await
		);

		// Write the token
		global_config::mutate_project(|x| x.tokens.cloud = Some(token)).await?;

		Ok(Output {
			output: "Token saved".to_string(),
		})
	}
}
