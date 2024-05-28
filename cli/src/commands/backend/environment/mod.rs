use clap::Parser;
use global_error::prelude::*;

pub mod create;

#[derive(Parser)]
pub enum SubCommand {
	Create(create::Opts),
}

impl SubCommand {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> GlobalResult<()> {
		match self {
			SubCommand::Create(opts) => opts.execute(ctx).await,
		}
	}
}
