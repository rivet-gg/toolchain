pub mod build;
pub mod clean;
pub mod config;
pub mod config_show;
pub mod create;
pub mod db;
pub mod deploy;
pub mod dev;
pub mod format;
pub mod init;
pub mod lint;
pub mod login;
pub mod logout;
pub mod sdk;
pub mod task;
pub mod test;

use clap::Parser;
use std::process::ExitCode;

#[derive(Parser)]
pub enum SubCommand {
	Login(login::Opts),
	Logout(logout::Opts),
	Deploy(deploy::Opts),
	Dev(dev::Opts),
	Config {
		#[clap(subcommand)]
		subcommand: config::SubCommand,
	},
	Task {
		#[clap(subcommand)]
		subcommand: task::SubCommand,
	},
	Build(build::Opts),
	Clean(clean::Opts),
	ConfigShow(config_show::Opts),
	Create {
		#[clap(subcommand)]
		subcommand: create::SubCommand,
	},
	Db {
		#[clap(subcommand)]
		subcommand: db::SubCommand,
	},
	Format(format::Opts),
	Init(init::Opts),
	Lint(lint::Opts),
	Sdk(sdk::Opts),
	Test(test::Opts),
}

impl SubCommand {
	pub async fn execute(&self) -> ExitCode {
		match self {
			SubCommand::Login(opts) => opts.execute().await,
			SubCommand::Logout(opts) => opts.execute().await,
			SubCommand::Deploy(opts) => opts.execute().await,
			SubCommand::Dev(opts) => opts.execute().await,
			SubCommand::Config { subcommand } => subcommand.execute().await,
			SubCommand::Task { subcommand } => subcommand.execute().await,
			SubCommand::Build(opts) => opts.execute().await,
			SubCommand::Clean(opts) => opts.execute().await,
			SubCommand::ConfigShow(opts) => opts.execute().await,
			SubCommand::Create { subcommand } => subcommand.execute().await,
			SubCommand::Db { subcommand } => subcommand.execute().await,
			SubCommand::Format(opts) => opts.execute().await,
			SubCommand::Init(opts) => opts.execute().await,
			SubCommand::Lint(opts) => opts.execute().await,
			SubCommand::Sdk(opts) => opts.execute().await,
			SubCommand::Test(opts) => opts.execute().await,
		}
	}
}
