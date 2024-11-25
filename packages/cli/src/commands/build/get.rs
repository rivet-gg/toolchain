use anyhow::*;
use clap::Parser;
use std::process::ExitCode;
use toolchain::rivet_api::apis;
use uuid::Uuid;

#[derive(Parser)]
pub struct Opts {
	#[clap(index = 1)]
	id: String,

	#[clap(long, alias = "env", short = 'e')]
	environment: Option<String>,
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

		let env = crate::util::env::get_or_select(&ctx, self.environment.as_ref()).await?;

		let build_id = Uuid::parse_str(&self.id).context("invalid id uuid")?;

		match apis::actor_builds_api::actor_builds_get(
			&ctx.openapi_config_cloud,
			&build_id.to_string(),
			Some(&ctx.project.name_id),
			Some(&env),
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
