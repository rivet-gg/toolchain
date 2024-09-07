pub mod format;
pub mod lint;
pub mod test;

use clap::Subcommand;
use std::process::ExitCode;

#[derive(Subcommand)]
pub enum SubCommand {
	Format(format::Opts),
	Lint(lint::Opts),
	Test(test::Opts),
}

impl SubCommand {
	pub async fn execute(&self) -> ExitCode {
		match &self {
			SubCommand::Format(opts) => opts.execute().await,
			SubCommand::Lint(opts) => opts.execute().await,
			SubCommand::Test(opts) => opts.execute().await,
		}
	}
}
