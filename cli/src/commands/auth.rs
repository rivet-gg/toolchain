use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
pub enum SubCommand {
	Token,
}

impl SubCommand {
	pub async fn execute(&self, ctx: &rivetctl::Ctx) -> Result<()> {
		match self {
			SubCommand::Token { .. } => {
				print!("Auth token: ");

				// Read token from stdin
				let token = tokio::task::spawn_blocking(|| {
					use std::io::BufRead;

					let stdin = std::io::stdin();
					let mut iterator = stdin.lock().lines();
					iterator.next().unwrap().context("token not provided")
				})
				.await??;

				// Create new config
				let mut new_config = ctx.config.clone();
				new_config.auth.token = Some(token.trim().to_owned());

				// Create new context
				let new_ctx = rivetctl::ctx::init(
					new_config.clone(),
					ctx.override_api_url.clone(),
					// Exclude overridden access token to check the token
					None,
				)
				.await?;
				let inspect = new_ctx
					.http_client
					.inspect()
					.send()
					.await
					.context("http_client.inspect")?;
				println!("{:?}", inspect);

				// Save new config
				let config_path = rivetctl::config::global::get_config_path()?;
				rivetctl::config::global::write(&new_config, &config_path).await?;
			}
		}

		Ok(())
	}
}
