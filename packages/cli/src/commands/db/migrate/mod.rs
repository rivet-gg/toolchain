pub mod drop;
pub mod push;

use clap::Subcommand;
use std::process::ExitCode;

#[derive(Subcommand)]
pub enum SubCommand {
	Push(push::Opts),
	Drop(drop::Opts),
}

impl SubCommand {
	pub async fn execute(&self) -> ExitCode {
		match &self {
			SubCommand::Push(opts) => opts.execute().await,
			SubCommand::Drop(opts) => opts.execute().await,
		}
	}
}
