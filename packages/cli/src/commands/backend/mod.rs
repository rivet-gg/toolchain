pub mod dev;

use clap::Subcommand;
use std::process::ExitCode;

#[derive(Subcommand)]
pub enum SubCommand {
	#[clap(alias = "dev")]
	Develop(dev::Opts),
}

impl SubCommand {
	pub async fn execute(&self) -> ExitCode {
		match self {
			SubCommand::Develop(opts) => opts.execute().await,
		}
	}
}
