use clap::Parser;
use std::process::ExitCode;
use toolchain::{
	tasks::{check_login_state, start_device_link, wait_for_login, RunConfig},
	util::task::run_task,
};

#[derive(Parser)]
pub struct Opts {
	#[clap(long, default_value = "https://api.rivet.gg")]
	api_endpoint: String,
}

impl Opts {
	pub async fn execute(&self) -> ExitCode {
		let run_config = RunConfig::empty();

		// Check if linked
		match run_task::<check_login_state::Task>(run_config.clone(), check_login_state::Input {})
			.await
		{
			Ok(output) => {
				if output.logged_in {
					eprintln!("Already logged in. Sign out with `rivet unlink`.");
					return ExitCode::SUCCESS;
				}
			}
			Err(e) => {
				eprintln!("Error checking login state: {}", e);
				return ExitCode::from(1);
			}
		}

		// Start device link
		let device_link_output = match run_task::<start_device_link::Task>(
			run_config.clone(),
			start_device_link::Input {
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
		match run_task::<wait_for_login::Task>(
			run_config.clone(),
			wait_for_login::Input {
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
