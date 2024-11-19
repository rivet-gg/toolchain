use clap::Parser;
use std::process::ExitCode;
use toolchain::tasks::{deploy, get_bootstrap_data};

use crate::util::task::{run_task, TaskOutputStyle};

/// Build & upload the game server & backend
#[derive(Parser)]
pub struct Opts {
	environment: String,
}

impl Opts {
	pub async fn execute(&self) -> ExitCode {
		let bootstrap_data = match run_task::<get_bootstrap_data::Task>(
			TaskOutputStyle::None,
			get_bootstrap_data::Input {},
		)
		.await
		{
			Ok(x) => x,
			Err(e) => {
				eprintln!("Error getting bootstrap: {e}");
				return ExitCode::FAILURE;
			}
		};
		let Some(cloud_data) = bootstrap_data.cloud else {
			eprintln!("Not signed in");
			return ExitCode::FAILURE;
		};

		// Find environment
		let environment = match cloud_data
			.envs
			.iter()
			.find(|env| env.slug == self.environment)
		{
			Some(env) => env,
			None => {
				eprintln!(
					"Environment '{}' not found. Available environments:",
					self.environment
				);
				for env in &cloud_data.envs {
					eprintln!("- {}", env.slug);
				}
				return ExitCode::FAILURE;
			}
		};

		let config = match toolchain::config::Config::load(None).await {
			Ok(x) => x,
			Err(e) => {
				eprintln!("Failed to load config: {e}");
				return ExitCode::FAILURE;
			}
		};

		match run_task::<deploy::Task>(
			TaskOutputStyle::Plain,
			deploy::Input {
				config,
				environment_id: environment.id,
			},
		)
		.await
		{
			Ok(_) => ExitCode::SUCCESS,
			Err(e) => {
				eprintln!("Error during deployment: {e}");
				ExitCode::FAILURE
			}
		}
	}
}
