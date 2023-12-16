use clap::Parser;
use global_error::prelude::*;

use crate::util::term;

#[derive(Parser)]
pub enum SubCommand {
	/// Deprecated.
	#[clap(hide = true)]
	CreateDevToken(crate::commands::token::create::dev::Opts),
}

impl SubCommand {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> GlobalResult<()> {
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
