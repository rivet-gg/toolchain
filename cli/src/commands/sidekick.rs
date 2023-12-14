use anyhow::{bail, Context, Result};
use clap::Parser;
use cli_core::{
	ctx,
	rivet_api::{self},
};
use console::Term;
use serde::Serialize;
use serde_json::{json, Value};
use url::Url;

use crate::util::{
	global_config,
	struct_fmt::{self, Format},
};

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
enum SideKickResponse {
	Ok(Value),
	Err(Value),
}

impl SubCommand {
	pub async fn get_link(&self) -> Result<()> {
		let (api_endpoint, _token) = global_config::read_project(|x| {
			(x.cluster.api_endpoint.clone(), x.tokens.cloud.clone())
		})
		.await?;

		// Create OpenAPI configuration without bearer token to send link request
		let openapi_config_cloud_unauthed = rivet_api::apis::configuration::Configuration {
			base_path: api_endpoint
				.clone()
				.unwrap_or_else(|| ctx::DEFAULT_API_ENDPOINT.to_string()),
			user_agent: Some(ctx::user_agent()),
			..Default::default()
		};

		// Prepare the link
		let prepare_res = rivet_api::apis::cloud_devices_links_api::cloud_devices_links_prepare(
			&openapi_config_cloud_unauthed,
		)
		.await;
		if let Err(err) = prepare_res.as_ref() {
			struct_fmt::print(
				&Format::Json,
				&SideKickResponse::Err(json!({
					"error": err.to_string(),
				})),
			)?;
			bail!("Error: {err:?}");
		}
		let prepare_res = prepare_res.context("cloud_devices_links_prepare")?;

		struct_fmt::print(
			&Format::Json,
			&SideKickResponse::Ok(json!({
				"device_link_url": prepare_res.device_link_url,
				"device_link_token": prepare_res.device_link_token,
			})),
		)?;

		Ok(())
	}

	pub async fn wait_for_login(&self, device_link_token: &String) -> Result<()> {
		let (api_endpoint, _token) = global_config::read_project(|x| {
			(x.cluster.api_endpoint.clone(), x.tokens.cloud.clone())
		})
		.await?;

		// Create OpenAPI configuration without bearer token to send link request
		let openapi_config_cloud_unauthed = rivet_api::apis::configuration::Configuration {
			base_path: api_endpoint
				.clone()
				.unwrap_or_else(|| ctx::DEFAULT_API_ENDPOINT.to_string()),
			user_agent: Some(ctx::user_agent()),
			..Default::default()
		};

		// Wait for link to complete
		let mut watch_index = None;
		let token = loop {
			let prepare_res = rivet_api::apis::cloud_devices_links_api::cloud_devices_links_get(
				&openapi_config_cloud_unauthed,
				&device_link_token,
				watch_index.as_ref().map(String::as_str),
			)
			.await;
			if let Err(err) = prepare_res.as_ref() {
				struct_fmt::print(
					&Format::Json,
					&SideKickResponse::Err(json!({
						"error": err.to_string(),
					})),
				)?;
				bail!("Error: {err:?}");
			}
			let prepare_res = prepare_res.context("cloud_devices_links_get")?;

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
			rivet_api::apis::cloud_auth_api::cloud_auth_inspect(&new_ctx.openapi_config_cloud)
				.await;
		if let Err(err) = inspect_res.as_ref() {
			struct_fmt::print(
				&Format::Json,
				&SideKickResponse::Err(json!({
					"error": err.to_string(),
				})),
			)?;

			bail!("Error: {err:?}");
		}
		let inspect_res = inspect_res.context("cloud_auth_inspect")?;

		// Find the game ID
		let Some(game_cloud) = inspect_res.agent.game_cloud.as_ref() else {
			struct_fmt::print(
				&Format::Json,
				&SideKickResponse::Err(json!({
					"error": "token is not a GameCloud token",
				})),
			)?;
			bail!("token is not a GameCloud token")
		};
		let game_id = game_cloud.game_id;

		// Extract game data
		let game_res = rivet_api::apis::cloud_games_games_api::cloud_games_games_get_game_by_id(
			&new_ctx.openapi_config_cloud,
			&game_id.to_string(),
			None,
		)
		.await;
		if let Err(err) = game_res.as_ref() {
			struct_fmt::print(
				&Format::Json,
				&SideKickResponse::Err(json!({
					"error": err.to_string(),
				})),
			)?;
			bail!("Error: {err:?}");
		}
		let game_res = game_res.context("cloud_games_games_get_game_by_id")?;

		// Write the token
		global_config::mutate_project(|x| x.tokens.cloud = Some(token)).await?;

		struct_fmt::print(
			&Format::Json,
			&SideKickResponse::Ok(json!({
				"output": "Token Saved"
			})),
		)?;

		Ok(())
	}

	pub async fn execute(&self, ctx: &cli_core::Ctx, _term: &Term) -> Result<()> {
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
			SubCommand::GetToken => {
				struct_fmt::print(
					&Format::Json,
					&SideKickResponse::Ok(json!({
						"token": ctx.access_token,
					})),
				)?;
			}
			SubCommand::GetVersion { namespace } => {
				// Get the game ID
				let game_res =
					rivet_api::apis::cloud_games_games_api::cloud_games_games_get_game_by_id(
						&ctx.openapi_config_cloud,
						&ctx.game_id,
						None,
					)
					.await
					.context("cloud_games_games_get_game_by_id")?;
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

				struct_fmt::print(
					&Format::Json,
					&SideKickResponse::Ok(json!({
						"output": parsed_url.to_string(),
					})),
				)?;
			}
		}

		Ok(())
	}

	pub fn validate_token(&self, token: &Option<String>) -> Result<()> {
		if token.is_none() {
			struct_fmt::print(
				&Format::Json,
				&SideKickResponse::Err(json!({
					"error": "No Rivet token found, please do the sign in process",
				})),
			)?;
		}

		Ok(())
	}
}
