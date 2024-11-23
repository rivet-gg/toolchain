pub mod create;
pub mod destroy;
pub mod get;
pub mod list;
pub mod logs;

use clap::Subcommand;
use std::process::ExitCode;

#[derive(Subcommand)]
pub enum SubCommand {
	Create(create::Opts),
	Get(get::Opts),
	Destroy(destroy::Opts),
	List(list::Opts),
	Logs(logs::Opts),
}

impl SubCommand {
	pub async fn execute(&self) -> ExitCode {
		match &self {
			SubCommand::Create(opts) => opts.execute().await,
			SubCommand::Get(opts) => opts.execute().await,
			SubCommand::Destroy(opts) => opts.execute().await,
			SubCommand::List(opts) => opts.execute().await,
			SubCommand::Logs(opts) => opts.execute().await,
		}
	}
}
