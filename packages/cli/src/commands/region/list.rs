use anyhow::*;
use clap::Parser;
use std::process::ExitCode;
use toolchain::rivet_api::apis;

#[derive(Parser)]
pub struct Opts {
	#[clap(index = 1)]
	environment: String,
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

		match apis::actor_regions_api::actor_regions_list(
			&ctx.openapi_config_cloud,
			Some(&ctx.project.name_id),
			Some(&self.environment),
		)
		.await
		{
			Result::Ok(res) => {
				println!("{:#?}", res.regions);
				Ok(ExitCode::SUCCESS)
			}
			Err(e) => {
				eprintln!("Failed to list regions: {}", e);
				Ok(ExitCode::FAILURE)
			}
		}
	}
}
