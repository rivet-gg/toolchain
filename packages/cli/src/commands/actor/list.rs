use anyhow::*;
use clap::Parser;
use std::{collections::HashMap, process::ExitCode};
use toolchain::rivet_api::apis;

use crate::util::kv_str;

#[derive(Parser)]
pub struct Opts {
	#[clap(index = 1)]
	environment: String,

	#[clap(long)]
	tags: Option<String>,

	#[clap(long)]
	include_destroyed: bool,

	#[clap(long)]
	cursor: Option<String>,
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

		// Parse tags
		let tags = self
			.tags
			.as_ref()
			.map(|tags_str| kv_str::from_str::<HashMap<String, String>>(tags_str))
			.transpose()?;
		let tags_json = tags.map(|t| serde_json::to_string(&t)).transpose()?;

		match apis::actor_api::actor_list(
			&ctx.openapi_config_cloud,
			Some(&ctx.project.name_id),
			Some(&self.environment),
			tags_json.as_deref(),
			Some(self.include_destroyed),
			self.cursor.as_deref(),
		)
		.await
		{
			Result::Ok(res) => {
				println!("{:#?}", res.actors);
				Ok(ExitCode::SUCCESS)
			}
			Err(e) => {
				eprintln!("Failed to list actors: {}", e);
				Ok(ExitCode::FAILURE)
			}
		}
	}
}
