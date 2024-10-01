use clap::Parser;
use std::process::ExitCode;
use toolchain::paths;

/// Outputs the path to where the data is stored for this project.
#[derive(Parser)]
pub struct Opts {}

impl Opts {
	pub async fn execute(&self) -> ExitCode {
		println!(
			"{}",
			paths::project_data_dir(&paths::data_dir().expect("data_dir"))
				.expect("project_data_dir")
				.display()
		);
		ExitCode::SUCCESS
	}
}
