use clap::Parser;
use std::process::ExitCode;
use toolchain::{
	tasks::{deploy, get_bootstrap_data, RunConfig},
	util::task::run_task,
};

#[derive(Parser)]
pub struct Opts {
	environment: String,
	#[clap(long, conflicts_with = "only_backend")]
	only_game_server: bool,
	#[clap(long, conflicts_with = "only_game_server")]
	only_backend: bool,
	#[clap(long)]
	backend_skip_migrate: bool,
}

impl Opts {
	pub async fn execute(&self) -> ExitCode {
		let run_config = RunConfig::empty();

		let bootstrap_data = match run_task::<get_bootstrap_data::Task>(
			run_config.clone(),
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

		// Find environment
		let environment = match bootstrap_data
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
				for env in &bootstrap_data.envs {
					eprintln!("- {}", env.slug);
				}
				return ExitCode::FAILURE;
			}
		};

		match run_task::<deploy::Task>(
			run_config,
			deploy::Input {
				cwd: std::env::current_dir()
					.unwrap_or_default()
					.to_string_lossy()
					.to_string(),
				environment_id: environment.id,
				game_server: !self.only_backend,
				backend: !self.only_game_server,
				backend_skip_migrate: self.backend_skip_migrate,
			},
		)
		.await
		{
			Ok(_) => {
				println!("Deployment completed successfully.");
				ExitCode::SUCCESS
			}
			Err(e) => {
				eprintln!("Error during deployment: {e}");
				ExitCode::FAILURE
			}
		}
	}
}
