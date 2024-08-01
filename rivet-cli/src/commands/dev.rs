use clap::Parser;
use global_error::prelude::*;

#[derive(Parser)]
pub enum SubCommand {
	/// Deprecated.
	#[clap(hide = true)]
	CreateDevToken(crate::commands::token::create::dev::Opts),
}

impl SubCommand {
	pub async fn execute(&self, ctx: &toolchain_core::Ctx) -> GlobalResult<()> {
		match self {
			SubCommand::CreateDevToken(opts) => {
				rivet_term::status::warn(
					"This command is deprecated. ",
					"Please use `rivet token create dev` instead.",
				);

				opts.execute(ctx).await
			}
		}
	}
}
