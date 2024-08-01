use clap::Parser;
use global_error::prelude::*;

pub mod unreal;

#[derive(Parser)]
pub enum SubCommand {
	Unreal {
		#[clap(subcommand)]
		command: unreal::SubCommand,
	},
}

impl SubCommand {
	pub async fn execute(&self, ctx: &toolchain_core::Ctx) -> GlobalResult<()> {
		match self {
			SubCommand::Unreal { command } => command.execute(ctx).await,
		}
	}
}
