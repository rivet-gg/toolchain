use clap::Subcommand;
use std::process::ExitCode;

mod list;

#[derive(Subcommand)]
pub enum SubCommand {
	List(list::Opts),
}

impl SubCommand {
	pub async fn execute(&self) -> ExitCode {
		match &self {
			SubCommand::List(opts) => opts.execute().await,
		}
	}
}
