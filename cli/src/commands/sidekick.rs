use anyhow::{Context, Result};
use clap::Parser;
use cli_core::{
	ctx,
	rivet_api::{self},
};
use console::Term;
use serde::Serialize;

use anyhow::bail;

use console::style;

use crate::util::{
	internal_config,
	struct_fmt::{self, Format},
	term,
};

#[derive(Parser)]
pub enum SubCommand {
	/// Go through the sign in process
	SignIn,
	/// Check if the CLI is logged in already
	CheckLoginState,
}

/// Any response that can come from the sidekick. There should only be a single
/// response from any sidekick call, though it might include multiple messages.
/// This is so a single schema can be parsed by whatever is consuming the
/// sidekick output.
#[derive(Serialize)]
enum SideKickResponse {
	Ok(SideKickResponseOk),
	Err(SideKickResponseErr),
}

/// The response from a sidekick call that was successful.
#[derive(Serialize)]
struct SideKickResponseOk {
	// The messages from the sidekick call.
	// messages: Vec<SideKickMessage>,
}

/// The response from a sidekick call that was unsuccessful.
#[derive(Serialize)]
enum SideKickResponseErr {
	APIError(String),
}

impl SubCommand {
	pub async fn execute_sign_in(&self, _term: &Term) -> Result<()> {
		let (api_endpoint, _token) =
			internal_config::read(|x| (x.cluster.api_endpoint.clone(), x.tokens.cloud.clone()))
				.await?;
		// Call the
		dbg!("read_token with override_endpoint: {:?}", &api_endpoint);
		// Create OpenAPI configuration without bearer token to send link request
		let openapi_config_cloud_unauthed = rivet_api::apis::configuration::Configuration {
			base_path: api_endpoint
				.clone()
				.unwrap_or_else(|| ctx::DEFAULT_API_ENDPOINT.to_string()),
			user_agent: Some(ctx::user_agent()),
			..Default::default()
		};
		dbg!(&openapi_config_cloud_unauthed);

		// Prepare the link
		let prepare_res = rivet_api::apis::cloud_devices_links_api::cloud_devices_links_prepare(
			&openapi_config_cloud_unauthed,
		)
		.await;
		if let Err(err) = prepare_res.as_ref() {
			struct_fmt::print(
				&Format::Json,
				&SideKickResponse::Err(SideKickResponseErr::APIError(format!("{err:?}"))),
			)?;
		}
		let prepare_res = prepare_res.context("cloud_devices_links_prepare")?;

		// Open link in browser
		if webbrowser::open_browser_with_options(
			webbrowser::Browser::Default,
			&prepare_res.device_link_url,
			webbrowser::BrowserOptions::new().with_suppress_output(true),
		)
		.is_ok()
		{
			term::status::info(
				"Waiting for link",
				"Select the game to link in your browser",
			);
		} else {
			// TODO: The case where the browser fails to open is not
			// handled well from Godot
			eprintln!(
				"{}\n  {}",
				style("Visit the link below").bold().blue(),
				style(&prepare_res.device_link_url)
					.italic()
					.underlined()
					.cyan()
			);
		}

		// Wait for link to complete
		let mut watch_index = None;
		let token = loop {
			let prepare_res = rivet_api::apis::cloud_devices_links_api::cloud_devices_links_get(
				&openapi_config_cloud_unauthed,
				&prepare_res.device_link_token,
				watch_index.as_ref().map(String::as_str),
			)
			.await;
			if let Err(err) = prepare_res.as_ref() {
				println!("Error: {err:?}");
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
			println!("Error: {err:?}");
		}
		let inspect_res = inspect_res.context("cloud_auth_inspect")?;

		// Find the game ID
		let Some(game_cloud) = inspect_res.agent.game_cloud.as_ref() else {
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
			println!("Error: {err:?}");
		}
		let game_res = game_res.context("cloud_games_games_get_game_by_id")?;
		let display_name = game_res.game.display_name;

		// Write the token
		internal_config::mutate(|x| x.tokens.cloud = Some(token)).await?;

		term::status::success("Token Saved", display_name);

		// Ok(new_ctx)

		// #[derive(Serialize)]
		// struct Output<'a> {
		// 	game_id: &'a str,
		// }
		// struct_fmt::print(format, &Output { game_id: &game_id })?;

		Ok(())
	}

	pub async fn execute(&self, _ctx: &cli_core::Ctx, _term: &Term) -> Result<()> {
		dbg!("");
		let (_api_endpoint, _token) =
			internal_config::read(|x| (x.cluster.api_endpoint.clone(), x.tokens.cloud.clone()))
				.await?;

		match self {
			SubCommand::SignIn => unreachable!("SignIn should be handled before this"),
			SubCommand::CheckLoginState => todo!(),
		}

		Ok(())
	}
}
