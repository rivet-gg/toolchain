use clap::Parser;
use std::process::ExitCode;
use toolchain::{backend, rivet_api::apis};

/// Get the current game version
#[derive(Parser)]
pub struct Opts {
	environment: String,
}

impl Opts {
	pub async fn execute(&self) -> ExitCode {
		let ctx = match toolchain::toolchain_ctx::load().await {
			Ok(x) => x,
			Err(err) => {
				eprintln!("failed to get ctx: {err}");
				return ExitCode::FAILURE;
			}
		};

		let env = match apis::cloud_games_api::cloud_games_get_game_by_id(
			&ctx.openapi_config_cloud,
			&ctx.game_id.to_string(),
			None,
		)
		.await
		{
			Ok(x) => x,
			Err(err) => {
				eprintln!("failed to get environments: {err}");
				return ExitCode::FAILURE;
			}
		};

		if let Some(env) = env
			.game
			.namespaces
			.iter()
			.find(|x| x.name_id == self.environment)
		{
			match backend::get_or_create_backend(&ctx, env.namespace_id).await {
				Ok(x) => {
					eprintln!("{}", x.endpoint);
					ExitCode::SUCCESS
				}
				Err(err) => {
					eprintln!("failed to get backend: {err}");
					ExitCode::FAILURE
				}
			}
		} else {
			eprintln!("environment not found");
			ExitCode::FAILURE
		}
	}
}
