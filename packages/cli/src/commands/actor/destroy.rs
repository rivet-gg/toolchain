use anyhow::*;
use clap::Parser;
use std::process::ExitCode;
use toolchain::rivet_api::apis;
use uuid::Uuid;

#[derive(Parser)]
pub struct Opts {
	#[clap(index = 1)]
	environment: String,

	#[clap(long)]
	id: String,

	#[clap(long)]
	override_kill_timeout: Option<i64>,
}

impl Opts {
	pub async fn execute(&self) -> ExitCode {
		match self.execute_inner().await {
			Result::Ok(code) => code,
			Err(err) => {
				eprintln!("{err}");
				ExitCode::FAILURE
			}
		}
	}

	pub async fn execute_inner(&self) -> Result<ExitCode> {
		let ctx = toolchain::toolchain_ctx::load().await?;

		let actor_id = Uuid::parse_str(&self.id).context("invalid id uuid")?;

		match apis::actor_api::actor_destroy(
			&ctx.openapi_config_cloud,
			&actor_id.to_string(),
			Some(&ctx.project.name_id),
			Some(&self.environment),
			self.override_kill_timeout,
		)
		.await
		{
			Result::Ok(_) => {
				println!("Destroyed actor: {actor_id}");
				Ok(ExitCode::SUCCESS)
			}
			Err(e) => {
				eprintln!("Failed to destroy actor: {}", e);
				Ok(ExitCode::FAILURE)
			}
		}
	}
}
