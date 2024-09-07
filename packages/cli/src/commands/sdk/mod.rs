pub mod generate;

use clap::Subcommand;
use std::process::ExitCode;

/// Manage the Rivet SDK
#[derive(Subcommand)]
pub enum SubCommand {
	Generate(generate::Opts),
}

impl SubCommand {
	pub async fn execute(&self) -> ExitCode {
		match &self {
			SubCommand::Generate(opts) => opts.execute().await,
        }
	}
}
