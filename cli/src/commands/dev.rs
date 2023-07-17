use anyhow::Result;
use clap::Parser;

use crate::util::term;

#[derive(Parser)]
pub enum SubCommand {
	/// Deprecated.
	CreateDevToken(crate::commands::token::create::dev::Opts),
}

impl SubCommand {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> Result<()> {
		match self {
			SubCommand::CreateDevToken(opts) => {
				term::status::warn(
					"This command is deprecated. ",
					"Please use `rivet token create dev` instead.",
				);

				opts.execute(ctx).await
			}
		}
	}
}
