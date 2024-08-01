use clap::Parser;
use global_error::prelude::*;

pub mod dev;
pub mod pub_ns;

#[derive(Parser)]
pub enum SubCommand {
	/// Creates a development token
	///
	/// https://rivet.gg/docs/general/concepts/token-types#namespace-development
	#[clap(alias = "dev")]
	Development(dev::Opts),

	/// Creates a public token
	///
	/// https://rivet.gg/docs/general/concepts/token-types#namespace-development
	#[clap(alias = "pub")]
	Public(pub_ns::Opts),
}

impl SubCommand {
	pub async fn execute(&self, ctx: &toolchain_core::Ctx) -> GlobalResult<()> {
		match self {
			SubCommand::Development(opts) => opts.execute(ctx).await,
			SubCommand::Public(opts) => opts.execute(ctx).await,
		}
	}
}
