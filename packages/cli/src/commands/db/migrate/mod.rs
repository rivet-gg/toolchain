pub mod apply;
pub mod drop;
pub mod generate;
pub mod push;

use clap::Subcommand;
use std::process::ExitCode;

/// Manage changes to the database schema
#[derive(Subcommand)]
pub enum SubCommand {
	Apply(apply::Opts),
	Push(push::Opts),
	Generate(generate::Opts),
	Drop(drop::Opts),
}

impl SubCommand {
	pub async fn execute(&self) -> ExitCode {
		match &self {
			SubCommand::Apply(opts) => opts.execute().await,
			SubCommand::Push(opts) => opts.execute().await,
			SubCommand::Generate(opts) => opts.execute().await,
			SubCommand::Drop(opts) => opts.execute().await,
		}
	}
}
