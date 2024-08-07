pub mod edit;

use clap::Subcommand;
use std::process::ExitCode;

#[derive(Subcommand)]
pub enum SubCommand {
	/// Edit the configuration file
	Edit(edit::Opts),
}

impl SubCommand {
	pub async fn execute(&self) -> ExitCode {
		match self {
			SubCommand::Edit(opts) => opts.execute().await,
		}
	}
}
