use clap::Parser;
use std::process::ExitCode;
use toolchain::tasks;

use crate::util::task::{run_task, TaskOutputStyle};

/// Login to a game
#[derive(Parser)]
pub struct Opts {
	#[clap(long, default_value = "https://api.rivet.gg")]
	api_endpoint: String,
}

impl Opts {
	pub async fn execute(&self) -> ExitCode {
		// Check if linked
		match run_task::<tasks::auth::check_state::Task>(
			TaskOutputStyle::None,
			tasks::auth::check_state::Input {},
		)
		.await
		{
			Ok(output) => {
				if output.signed_in {
					eprintln!("Already logged in. Sign out with `rivet logout`.");
					return ExitCode::SUCCESS;
				}
			}
			Err(e) => {
				eprintln!("Error checking login state: {}", e);
				return ExitCode::from(1);
			}
		}

		// Start device link
		let device_link_output = match run_task::<tasks::auth::start_sign_in::Task>(
			TaskOutputStyle::None,
			tasks::auth::start_sign_in::Input {
				api_endpoint: self.api_endpoint.clone(),
			},
		)
		.await
		{
			Ok(output) => output,
			Err(e) => {
				eprintln!("Error starting device link: {}", e);
				return ExitCode::from(2);
			}
		};
		eprintln!("{}", device_link_output.device_link_url);

		// Wait for finish
		match run_task::<tasks::auth::wait_for_sign_in::Task>(
			TaskOutputStyle::None,
			tasks::auth::wait_for_sign_in::Input {
				api_endpoint: self.api_endpoint.clone(),
				device_link_token: device_link_output.device_link_token,
			},
		)
		.await
		{
			Ok(_) => {
				eprintln!("Logged in");
				ExitCode::SUCCESS
			}
			Err(e) => {
				eprintln!("Error waiting for login: {}", e);
				ExitCode::from(3)
			}
		}
	}
}
