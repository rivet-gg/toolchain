use clap::Parser;
use std::fs;
use std::process::ExitCode;
use toolchain::{
	tasks::{get_settings_paths, RunConfig},
	util::task::run_task,
};

#[derive(Parser)]
pub struct Opts {
	#[clap(subcommand)]
	subcommand: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
	User,
	Project,
}

impl Opts {
	pub async fn execute(&self) -> ExitCode {
		let run_config = RunConfig::empty();

		// Get settings paths
		let settings_paths =
			match run_task::<get_settings_paths::Task>(run_config, get_settings_paths::Input {})
				.await
			{
				Ok(output) => output,
				Err(e) => {
					eprintln!("Error getting settings paths: {}", e);
					return ExitCode::FAILURE;
				}
			};

		let path = match self.subcommand {
			SubCommand::User => settings_paths.user_path,
			SubCommand::Project => settings_paths.project_path,
		};

		// Ensure the file and its parent directories exist
		if let Some(parent) = path.parent() {
			if let Err(e) = fs::create_dir_all(parent) {
				eprintln!("Error creating parent directories: {}", e);
				return ExitCode::FAILURE;
			}
		}
		if !path.exists() {
			if let Err(e) = fs::write(&path, "{}") {
				eprintln!("Error creating settings file: {}", e);
				return ExitCode::FAILURE;
			}
		}

		// Open the settings file in the default editor
		if let Err(e) = open::that(&path) {
			eprintln!("Error opening settings file: {}", e);
			ExitCode::FAILURE
		} else {
			println!("Settings file opened in your default editor.");
			ExitCode::SUCCESS
		}
	}
}
