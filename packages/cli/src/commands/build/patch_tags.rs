use anyhow::*;
use clap::Parser;
use std::{collections::HashMap, process::ExitCode};
use toolchain::rivet_api::{apis, models};

use crate::util::kv_str;

#[derive(Parser)]
pub struct Opts {
	#[clap(index = 1)]
	environment: String,

	#[clap(index = 2)]
	build: String,

	#[clap(short = 't', long = "tag")]
	tags: Option<String>,

	#[clap(short = 'e', long = "exclusive-tag")]
	exclusive_tags: Option<String>,
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
		let exclusive_tags = self.exclusive_tags.as_ref().map(|x| {
			x.split(",")
				.map(|x| x.trim().to_string())
				.collect::<Vec<String>>()
		});

		match apis::actor_builds_api::actor_builds_patch_tags(
			&ctx.openapi_config_cloud,
			&self.build,
			models::ActorPatchBuildTagsRequest {
				tags: tags.map(|x| serde_json::json!(x)),
				exclusive_tags,
			},
			Some(&ctx.project.name_id),
			Some(&self.environment),
		)
		.await
		{
			Result::Ok(_) => {
				println!("Patched tags");
				Ok(ExitCode::SUCCESS)
			}
			Err(e) => {
				eprintln!("Failed to patch tags: {}", e);
				Ok(ExitCode::FAILURE)
			}
		}
	}
}
