use std::{collections::HashMap, process::ExitCode};
use toolchain::tasks::{deploy, get_bootstrap_data};
use uuid::Uuid;

use crate::util::task::{run_task, TaskOutputStyle};

pub struct DeployOpts<'a> {
	pub environment: &'a str,
	pub build_tags: Option<HashMap<String, String>>,
}

pub async fn deploy(opts: DeployOpts<'_>) -> Result<Vec<Uuid>, ExitCode> {
	let bootstrap_data = match run_task::<get_bootstrap_data::Task>(
		TaskOutputStyle::None,
		get_bootstrap_data::Input {},
	)
	.await
	{
		Ok(x) => x,
		Err(e) => {
			eprintln!("Error getting bootstrap: {e}");
			return Err(ExitCode::FAILURE);
		}
	};
	let Some(cloud_data) = bootstrap_data.cloud else {
		eprintln!("Not signed in. Please run `rivet login`.");
		return Err(ExitCode::FAILURE);
	};

	// Find environment
	let environment = match cloud_data
		.envs
		.iter()
		.find(|env| env.slug == opts.environment)
	{
		Some(env) => env,
		None => {
			eprintln!(
				"Environment '{}' not found. Available environments:",
				opts.environment
			);
			for env in &cloud_data.envs {
				eprintln!("- {}", env.slug);
			}
			return Err(ExitCode::FAILURE);
		}
	};

	let config = match toolchain::config::Config::load(None).await {
		Ok(x) => x,
		Err(e) => {
			eprintln!("Failed to load config: {e}");
			return Err(ExitCode::FAILURE);
		}
	};

	match run_task::<deploy::Task>(
		TaskOutputStyle::PlainNoResult,
		deploy::Input {
			config,
			environment_id: environment.id,
			build_tags: opts.build_tags,
		},
	)
	.await
	{
		Ok(build) => Ok(build.build_ids),
		Err(e) => {
			eprintln!("Error during deploy: {e}");
			Err(ExitCode::FAILURE)
		}
	}
}
