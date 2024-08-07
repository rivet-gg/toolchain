pub mod login;
pub mod logout;
pub mod task;
pub mod deploy;

use clap::Parser;
use std::process::ExitCode;

#[derive(Parser)]
pub enum SubCommand {
    Login(login::Opts),
    Task {
        #[clap(subcommand)]
        subcommand: task::SubCommand,
    },
    Logout(logout::Opts),
    Deploy(deploy::Opts),
}

impl SubCommand {
    pub async fn execute(&self) -> ExitCode {
        match self {
            SubCommand::Login(opts) => opts.execute().await,
            SubCommand::Task { subcommand } => subcommand.execute().await,
            SubCommand::Logout(opts) => opts.execute().await,
            SubCommand::Deploy(opts) => opts.execute().await,
        }
    }
}