use clap::Parser;
use cli_core::{
	ctx,
	rivet_api::{self, apis},
};
use console::Term;
use global_error::prelude::*;
use serde::Serialize;
use serde_json::{json, Value};
use url::Url;

use crate::util::global_config;

#[derive(Parser)]
pub enum SubCommand {
	/// Get the link for the user to sign in
	GetLink,
	/// Long poll the server to check if the user has signed in
	WaitForLogin {
		/// The token to poll for
		#[structopt(short, long)]
		device_link_token: String,
	},
	/// Check if the CLI is logged in already
	CheckLoginState,
	/// Get the token from the CLI
	GetToken,
	///
	GetVersion {
		/// The namespace to get the version for
		#[structopt(short, long)]
		namespace: String,
	},
}

/// Any response that can come from the sidekick. There should only be a single
/// response from any sidekick call, though it might include multiple messages.
/// This is so a single schema can be parsed by whatever is consuming the
/// sidekick output.
#[derive(Serialize)]
pub struct SideKickResponse(pub Value);

impl SubCommand {
	pub async fn get_link(&self) -> GlobalResult<SideKickResponse> {
		let (api_endpoint, _token) = unwrap!(
			global_config::read_project(|x| {
				(x.cluster.api_endpoint.clone(), x.tokens.cloud.clone())
			})
			.await
		);

		// Create OpenAPI configuration without bearer token to send link request
		let openapi_config_cloud_unauthed = apis::configuration::Configuration {
			base_path: api_endpoint
				.clone()
				.unwrap_or_else(|| ctx::DEFAULT_API_ENDPOINT.to_string()),
			user_agent: Some(ctx::user_agent()),
			..Default::default()
		};

		// Prepare the link
		let prepare_res = unwrap!(
			apis::cloud_devices_links_api::cloud_devices_links_prepare(
				&openapi_config_cloud_unauthed,
			)
			.await
		);

		Ok(SideKickResponse(json!({
			"device_link_url": prepare_res.device_link_url,
			"device_link_token": prepare_res.device_link_token,
		})))
	}

	pub async fn wait_for_login(
		&self,
		device_link_token: &String,
	) -> GlobalResult<SideKickResponse> {
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
					&device_link_token,
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
		let game_res = unwrap!(
			apis::cloud_games_games_api::cloud_games_games_get_game_by_id(
				&new_ctx.openapi_config_cloud,
				&game_id.to_string(),
				None,
			)
			.await
		);

		// Write the token
		global_config::mutate_project(|x| x.tokens.cloud = Some(token)).await?;

		Ok(SideKickResponse(json!({
			"output": "Token Saved"
		})))
	}

	pub async fn execute(
		&self,
		ctx: &cli_core::Ctx,
		_term: &Term,
	) -> GlobalResult<SideKickResponse> {
		let (_api_endpoint, _token) = global_config::read_project(|x| {
			(x.cluster.api_endpoint.clone(), x.tokens.cloud.clone())
		})
		.await?;

		match self {
			SubCommand::GetLink => unreachable!("GetLink should be handled before this"),
			SubCommand::WaitForLogin { .. } => {
				unreachable!("WaitForLogin should be handled before this")
			}
			SubCommand::CheckLoginState => todo!(),
			SubCommand::GetToken => Ok(SideKickResponse(json!({
				"token": ctx.access_token,
			}))),
			SubCommand::GetVersion { namespace } => {
				// Get the game ID
				let game_res = unwrap!(
					rivet_api::apis::cloud_games_games_api::cloud_games_games_get_game_by_id(
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
					ctx.api_endpoint, game_id, namespace
				);

				// Parse the URL and change the subdomain from `api` to `hub`
				let mut parsed_url = Url::parse(&url).unwrap();
				let host = parsed_url.host_str().unwrap().replace("api", "hub");
				parsed_url.set_host(Some(&host)).unwrap();

				Ok(SideKickResponse(json!({
					"output": parsed_url.to_string(),
				})))
			}
		}
	}

	pub fn validate_token(&self, token: &Option<String>) -> GlobalResult<SideKickResponse> {
		if token.is_none() {
			bail!("No Rivet token found, please do the sign in process");
		}

		Ok(SideKickResponse(json!({
			"output": "Token Valid",
		})))
	}
}
