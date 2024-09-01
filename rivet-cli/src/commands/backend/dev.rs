use clap::Parser;
use std::process::ExitCode;
use toolchain::tasks::backend_dev;

use crate::util::task::{run_task, TaskOutputStyle};

#[derive(Parser)]
pub struct Opts {}

impl Opts {
	pub async fn execute(&self) -> ExitCode {
		match run_task::<backend_dev::Task>(
			TaskOutputStyle::PlainNoResult,
			backend_dev::Input {
				port: 6420,
				cwd: std::env::current_dir()
					.unwrap_or_default()
					.to_string_lossy()
					.to_string(),
			},
		)
		.await
		{
			Ok(_) => ExitCode::SUCCESS,
			Err(e) => {
				eprintln!("Error during dev: {e}");
				ExitCode::FAILURE
			}
		}
	}
}
