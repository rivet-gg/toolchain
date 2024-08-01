pub mod login;
pub mod logout;
pub mod task;

use clap::Parser;
use global_error::prelude::*;

#[derive(Parser)]
pub enum SubCommand {
	Login(login::Opts),
	Task {
		#[clap(subcommand)]
		subcommand: task::SubCommand,
	},
	Logout(logout::Opts),
}

impl SubCommand {
	pub async fn execute(&self) -> GlobalResult<()> {
		match self {
			SubCommand::Login(opts) => opts.execute().await,
			SubCommand::Task { subcommand } => subcommand.execute().await,
			SubCommand::Logout(opts) => opts.execute().await,
		}
	}
}
