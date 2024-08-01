use clap::Parser;
use global_error::prelude::*;
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
	pub async fn execute(&self) -> GlobalResult<()> {
		let (run_config, _temp_dir) = RunConfig::with_temp_dir()?;

		// Check if linked
		let output =
			run_task::<check_login_state::Task>(run_config.clone(), check_login_state::Input {})
				.await?;
		if output.logged_in {
			eprintln!("Already logged in. Sign out with `rivet unlink`.");
			return Ok(());
		}

		// Start device link
		let output = run_task::<start_device_link::Task>(
			run_config.clone(),
			start_device_link::Input {
				api_endpoint: self.api_endpoint.clone(),
			},
		)
		.await?;
		eprintln!("{}", output.device_link_url);

		// Wait for finish
		run_task::<wait_for_login::Task>(
			run_config.clone(),
			wait_for_login::Input {
				api_endpoint: self.api_endpoint.clone(),
				device_link_token: output.device_link_token,
			},
		)
		.await?;
		eprintln!("Logged in");

		Ok(())
	}
}
