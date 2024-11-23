pub mod actor;
pub mod build;
pub mod deploy;
pub mod init;
pub mod metadata;
pub mod region;
pub mod sign_in;
pub mod sign_out;

use clap::Parser;
use std::process::ExitCode;

#[derive(Parser)]
pub enum SubCommand {
	Init(init::Opts),
	#[clap(alias = "login")]
	Signin(sign_in::Opts),
	#[clap(alias = "logout")]
	Signout(sign_out::Opts),
	#[clap(alias = "d")]
	Deploy(deploy::Opts),
	#[clap(alias = "a")]
	Actor {
		#[clap(subcommand)]
		subcommand: actor::SubCommand,
	},
	#[clap(alias = "b")]
	Build {
		#[clap(subcommand)]
		subcommand: build::SubCommand,
	},
	Region {
		#[clap(subcommand)]
		subcommand: region::SubCommand,
	},
	#[clap(alias = "meta")]
	Metadata {
		#[clap(subcommand)]
		subcommand: metadata::SubCommand,
	},
}

impl SubCommand {
	pub async fn execute(&self) -> ExitCode {
		match self {
			SubCommand::Init(opts) => opts.execute().await,
			SubCommand::Signin(opts) => opts.execute().await,
			SubCommand::Signout(opts) => opts.execute().await,
			SubCommand::Deploy(opts) => opts.execute().await,
			SubCommand::Actor { subcommand } => subcommand.execute().await,
			SubCommand::Build { subcommand } => subcommand.execute().await,
			SubCommand::Region { subcommand } => subcommand.execute().await,
			SubCommand::Metadata { subcommand } => subcommand.execute().await,
		}
	}
}
