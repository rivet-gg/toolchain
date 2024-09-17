pub mod build;
pub mod show;

use clap::Subcommand;
use std::process::ExitCode;

/// Manage the backend
#[derive(Subcommand)]
pub enum SubCommand {
	Build(build::Opts),
	Show(show::Opts),
}

impl SubCommand {
	pub async fn execute(&self) -> ExitCode {
		match &self {
			SubCommand::Build(opts) => opts.execute().await,
			SubCommand::Show(opts) => opts.execute().await,
		}
	}
}