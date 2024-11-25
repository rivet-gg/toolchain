use clap::Subcommand;
use std::process::ExitCode;

mod select;

#[derive(Subcommand)]
pub enum SubCommand {
	#[clap(alias = "s")]
	Select(select::Opts),
}

impl SubCommand {
	pub async fn execute(&self) -> ExitCode {
		match &self {
			SubCommand::Select(opts) => opts.execute().await,
		}
	}
}
