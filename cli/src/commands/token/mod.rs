use anyhow::Result;
use clap::Parser;

pub mod create;

#[derive(Parser)]
pub enum SubCommand {
	#[clap()]
	Create {
		#[clap(subcommand)]
		command: create::SubCommand,
	},
}

impl SubCommand {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> Result<()> {
		match self {
			SubCommand::Create { command } => command.execute(ctx).await,
		}
	}
}
