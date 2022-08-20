use anyhow::{bail, Context, Result};
use clap::Parser;
use std::io::{self, Write};

use crate::util::secrets;

#[derive(Parser)]
pub struct Opts {}

impl Opts {
	pub async fn execute(&self, override_api_url: Option<String>) -> Result<()> {
		// Check if token already exists
		if secrets::read_cloud_token().await?.is_some() {
			bail!("cloud token already exists")
		}

		print!("Cloud token: ");
		io::stdout().flush()?;

		// Read token from stdin
		let token = tokio::task::spawn_blocking(|| {
			use std::io::BufRead;

			let stdin = std::io::stdin();
			let mut iterator = stdin.lock().lines();
			iterator.next().unwrap().context("token not provided")
		})
		.await??;

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

		println!();
		println!("Display name: {display_name}");
		println!("Name ID: {name_id}");
		println!("Game ID: {game_id}");
		println!();

		// Write the token
		secrets::write_cloud_token(&token).await?;

		Ok(())
	}
}
