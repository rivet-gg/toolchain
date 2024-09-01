use clap::Parser;
use std::process::ExitCode;

/// EXPERIMENTAL
#[derive(Parser)]
pub enum SubCommand {
	Run(RunOpts),
}

impl SubCommand {
	pub async fn execute(&self) -> ExitCode {
		match self {
			SubCommand::Run(opts) => opts.execute().await,
		}
	}
}

#[derive(Parser)]
pub struct RunOpts {
	#[clap(long)]
	run_config: String,
	#[clap(long)]
	name: String,
	#[clap(long)]
	input: String,
}

impl RunOpts {
	pub async fn execute(&self) -> ExitCode {
		match serde_json::from_str(&self.run_config) {
			Ok(run_config) => {
				let result =
					toolchain::tasks::run_task_json(run_config, &self.name, &self.input).await;

				if result.success {
					ExitCode::SUCCESS
				} else {
					ExitCode::FAILURE
				}
			}
			Err(e) => {
				eprintln!("Error parsing run_config: {}", e);
				ExitCode::from(2)
			}
		}
	}
}
