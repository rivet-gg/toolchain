use anyhow::Result;
use clap::Parser;

pub mod dev;

#[derive(Parser)]
pub enum SubCommand {
	#[clap(alias = "dev")]
	Development(dev::Opts),
}

impl SubCommand {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> Result<()> {
		match self {
			SubCommand::Development(opts) => opts.execute(ctx).await,
		}
	}
}
