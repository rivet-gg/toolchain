use clap::Parser;
use std::process::ExitCode;
use toolchain::{
	tasks::{unlink, RunConfig},
	util::task::run_task,
};

#[derive(Parser)]
pub struct Opts {}

impl Opts {
	pub async fn execute(&self) -> ExitCode {
		let run_config = RunConfig::empty();

		match run_task::<unlink::Task>(run_config.clone(), unlink::Input {}).await {
			Ok(_) => {
				eprintln!("Logged out");
				ExitCode::SUCCESS
			}
			Err(e) => {
				eprintln!("Error logging out: {}", e);
				ExitCode::from(1)
			}
		}
	}
}
