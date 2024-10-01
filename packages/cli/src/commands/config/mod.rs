pub mod edit;
pub mod data_path;

use clap::Subcommand;
use std::process::ExitCode;

/// Manage Rivet configuration
#[derive(Subcommand)]
pub enum SubCommand {
	Edit(edit::Opts),
	DataPath(data_path::Opts),
}

impl SubCommand {
	pub async fn execute(&self) -> ExitCode {
		match self {
			SubCommand::Edit(opts) => opts.execute().await,
			SubCommand::DataPath(opts) => opts.execute().await,
		}
	}
}
