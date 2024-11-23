use clap::Subcommand;
use std::process::ExitCode;
use toolchain::paths;

#[derive(Subcommand)]
pub enum SubCommand {
	Path,
}

impl SubCommand {
	pub async fn execute(&self) -> ExitCode {
		match self {
			SubCommand::Path => {
				println!(
					"{}",
					paths::project_data_dir(&paths::data_dir().expect("data_dir"))
						.expect("project_data_dir")
						.display()
				);
				ExitCode::SUCCESS
			}
		}
	}
}
