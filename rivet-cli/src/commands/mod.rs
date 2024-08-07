pub mod config;
pub mod deploy;
pub mod login;
pub mod logout;
pub mod task;

use clap::Parser;
use std::process::ExitCode;

#[derive(Parser)]
pub enum SubCommand {
	Login(login::Opts),
	Logout(logout::Opts),
	Deploy(deploy::Opts),
	Config {
		#[clap(subcommand)]
		subcommand: config::SubCommand,
	},
	Task {
		#[clap(subcommand)]
		subcommand: task::SubCommand,
	},
}

impl SubCommand {
	pub async fn execute(&self) -> ExitCode {
		match self {
			SubCommand::Login(opts) => opts.execute().await,
			SubCommand::Logout(opts) => opts.execute().await,
			SubCommand::Deploy(opts) => opts.execute().await,
			SubCommand::Config { subcommand } => subcommand.execute().await,
			SubCommand::Task { subcommand } => subcommand.execute().await,
		}
	}
}
