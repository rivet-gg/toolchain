pub mod start;
pub mod status;
pub mod stop;

use clap::Subcommand;
use std::process::ExitCode;

/// Manage the development database
#[derive(Subcommand)]
pub enum SubCommand {
	Start(start::Opts),
	Status(status::Opts),
	Stop(stop::Opts),
}

impl SubCommand {
	pub async fn execute(&self) -> ExitCode {
		match &self {
			SubCommand::Start(opts) => opts.execute().await,
			SubCommand::Status(opts) => opts.execute().await,
			SubCommand::Stop(opts) => opts.execute().await,
		}
	}
}
