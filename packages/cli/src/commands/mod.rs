pub mod backend;
pub mod init;
pub mod dev;
pub mod clean;
pub mod config;
pub mod create;
pub mod db;
pub mod deploy;
pub mod login;
pub mod logout;
pub mod module;
pub mod sdk;

use clap::Parser;
use std::process::ExitCode;

#[derive(Parser)]
pub enum SubCommand {
	Init(login::Opts),
	Login(login::Opts),
	Logout(logout::Opts),
	Dev(dev::Opts),
	Deploy(deploy::Opts),
	Config {
		#[clap(subcommand)]
		subcommand: config::SubCommand,
	},
	Clean(clean::Opts),
	Create {
		#[clap(subcommand)]
		subcommand: create::SubCommand,
	},
	Db {
		#[clap(subcommand)]
		subcommand: db::SubCommand,
	},
	Sdk {
		#[clap(subcommand)]
		subcommand: sdk::SubCommand,
	},
	Backend {
		#[clap(subcommand)]
		subcommand: backend::SubCommand,
	},
	Module {
		#[clap(subcommand)]
		subcommand: module::SubCommand,
	},
}

impl SubCommand {
	pub async fn execute(&self) -> ExitCode {
		match self {
			SubCommand::Init(opts) => opts.execute().await,
			SubCommand::Login(opts) => opts.execute().await,
			SubCommand::Logout(opts) => opts.execute().await,
			SubCommand::Dev(opts) => opts.execute().await,
			SubCommand::Deploy(opts) => opts.execute().await,
			SubCommand::Config { subcommand } => subcommand.execute().await,
			SubCommand::Clean(opts) => opts.execute().await,
			SubCommand::Create { subcommand } => subcommand.execute().await,
			SubCommand::Db { subcommand } => subcommand.execute().await,
			SubCommand::Sdk { subcommand} => subcommand.execute().await,
			SubCommand::Backend { subcommand } => subcommand.execute().await,
			SubCommand::Module { subcommand } => subcommand.execute().await,
		}
	}
}
