use anyhow::{bail, Context, Result};
use clap::Parser;
use console::Term;

use crate::util::{secrets, term};

#[derive(Parser)]
pub struct Opts {}

impl Opts {
	pub async fn execute(&self, term: &Term, override_api_url: Option<String>) -> Result<()> {
		// Check if token already exists
		if secrets::read_cloud_token().await?.is_none() {
			read_cloud_token(term, override_api_url.clone()).await?;
		} else {
			term::success("Cloud token already exists");
		}

		Ok(())
	}
}

async fn read_cloud_token(term: &Term, override_api_url: Option<String>) -> Result<()> {
	term.write_line("Cloud token: ")?;
	let token = tokio::task::block_in_place(|| term.read_secure_line())?;

	// Create new context
	let new_ctx = cli_core::ctx::init(
		override_api_url,
		// Exclude overridden access token to check the token
		token.clone(),
	)
	.await?;
	let inspect = new_ctx
		.client()
		.inspect()
		.send()
		.await
		.context("client.inspect()")?;

	let game_id = match inspect.agent.as_ref().context("inspect.agent")? {
		cli_core::rivet_cloud::model::AuthAgent::GameCloud(game_cloud) => {
			game_cloud.game_id.clone().context("game_cloud.game_id")?
		}
		_ => bail!("invalid agent kind"),
	};

	let game_res = new_ctx
		.client()
		.get_game_by_id()
		.game_id(game_id)
		.send()
		.await
		.context("client.get_game_by_id()")?;
	let game = game_res.game().context("game_res.game")?;
	let game_id = game.game_id().context("game.game_id")?;
	let name_id = game.name_id().context("game.name_id")?;
	let display_name = game.display_name().context("game.display_name")?;

	eprintln!();
	eprintln!("Display name: {display_name}");
	eprintln!("Name ID: {name_id}");
	eprintln!("Game ID: {game_id}");
	eprintln!();

	// Write the token
	secrets::write_cloud_token(&token).await?;

	term::success("New cloud token saved");

	Ok(())
}
