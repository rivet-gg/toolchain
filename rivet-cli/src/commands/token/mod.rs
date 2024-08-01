use clap::Parser;
use global_error::prelude::*;

pub mod create;

#[derive(Parser)]
pub enum SubCommand {
	Create {
		#[clap(subcommand)]
		command: create::SubCommand,
	},
}

impl SubCommand {
	pub async fn execute(&self, ctx: &toolchain_core::Ctx) -> GlobalResult<()> {
		match self {
			SubCommand::Create { command } => command.execute(ctx).await,
		}
	}
}
