use anyhow::*;
use clap::Parser;
use std::{collections::HashMap, process::ExitCode};
use toolchain::rivet_api::apis;

#[derive(Parser)]
pub struct Opts {
	#[clap(long, alias = "env", short = 'e')]
	environment: Option<String>,

	#[clap(long)]
	tags: Option<String>,
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

		// Parse tags
		let tags = self
			.tags
			.as_ref()
			.map(|tags_str| kv_str::from_str::<HashMap<String, String>>(tags_str))
			.transpose()?;
		let tags_json = tags.map(|t| serde_json::to_string(&t)).transpose()?;

		match apis::actor_builds_api::actor_builds_list(
			&ctx.openapi_config_cloud,
			Some(&ctx.project.name_id),
			Some(&env),
			tags_json.as_deref(),
		)
		.await
		{
			Result::Ok(res) => {
				println!("{:#?}", res.builds);
				Ok(ExitCode::SUCCESS)
			}
			Err(e) => {
				eprintln!("Failed to list builds: {}", e);
				Ok(ExitCode::FAILURE)
			}
		}
	}
}
