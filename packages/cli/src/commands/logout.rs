use clap::Parser;
use std::process::ExitCode;
use toolchain::tasks;

use crate::util::task::{run_task, TaskOutputStyle};

/// Logout from a game
#[derive(Parser)]
pub struct Opts {}

impl Opts {
	pub async fn execute(&self) -> ExitCode {
		match run_task::<tasks::auth::sign_out::Task>(
			TaskOutputStyle::None,
			tasks::auth::sign_out::Input {},
		)
		.await
		{
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
