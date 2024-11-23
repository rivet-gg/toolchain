pub mod get_current_version;

use clap::Subcommand;
use std::process::ExitCode;

/// Manage the backend
#[derive(Subcommand)]
pub enum SubCommand {
	#[clap(name = "get-current-version")]
	GetCurrentVersion(get_current_version::Opts),
}

impl SubCommand {
	pub async fn execute(&self) -> ExitCode {
		match &self {
			SubCommand::GetCurrentVersion(opts) => opts.execute().await,
		}
	}
}
