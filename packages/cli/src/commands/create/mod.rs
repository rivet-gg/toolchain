pub mod module;
pub mod test;
pub mod script;

use clap::Subcommand;
use std::process::ExitCode;

#[derive(Subcommand)]
pub enum SubCommand {
	Module(module::Opts),
	Test(test::Opts),
	Script(script::Opts),
}

impl SubCommand {
	pub async fn execute(&self) -> ExitCode {
		match self {
			SubCommand::Module(opts) => opts.execute().await,
			SubCommand::Test(opts) => opts.execute().await,
			SubCommand::Script(opts) => opts.execute().await,
		}
	}
}
