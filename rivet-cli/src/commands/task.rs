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
	name: String,
	#[clap(long)]
	input: String,
}

impl RunOpts {
	pub async fn execute(&self) -> ExitCode {
		let result = crate::util::task::run_task_json(
			crate::util::task::TaskOutputStyle::Json,
			&self.name,
			&self.input,
		)
		.await;
		match result {
			Ok(res) => {
				if res.success {
					ExitCode::SUCCESS
				} else {
					ExitCode::FAILURE
				}
			}
			Err(err) => {
				eprintln!("error running task: {err:?}");
				ExitCode::FAILURE
			}
		}
	}
}
