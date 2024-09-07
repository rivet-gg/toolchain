pub mod reset;
pub mod sh;
pub mod url;
pub mod migrate;
pub mod instance;

use clap::Subcommand;
use std::process::ExitCode;

#[derive(Subcommand)]
pub enum SubCommand {
	Reset(reset::Opts),
	Sh(sh::Opts),
	Url(url::Opts),
	Migrate {
		#[clap(subcommand)]
		subcommand: migrate::SubCommand,
	},
	Instance {
		#[clap(subcommand)]
		subcommand: instance::SubCommand,
	},
}

impl SubCommand {
	pub async fn execute(&self) -> ExitCode {
		match self {
			SubCommand::Reset(opts) => opts.execute().await,
			SubCommand::Sh(opts) => opts.execute().await,
			SubCommand::Url(opts) => opts.execute().await,
			SubCommand::Migrate { subcommand} => subcommand.execute().await,
			SubCommand::Instance { subcommand} => subcommand.execute().await,
		}
	}
}
