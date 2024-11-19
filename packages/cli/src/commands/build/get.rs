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

		let build_id = Uuid::parse_str(&self.id).context("invalid id uuid")?;

		match apis::actor_builds_api::actor_builds_get(
			&ctx.openapi_config_cloud,
			&build_id.to_string(),
			Some(&ctx.project.name_id),
			Some(&self.environment),
		)
		.await
		{
			Result::Ok(res) => {
				println!("{:#?}", res.build);
				Ok(ExitCode::SUCCESS)
			}
			Err(e) => {
				eprintln!("Failed to get build: {}", e);
				Ok(ExitCode::FAILURE)
			}
		}
	}
}
