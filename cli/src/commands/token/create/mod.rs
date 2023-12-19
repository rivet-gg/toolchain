use anyhow::Result;
use clap::Parser;

pub mod dev;
pub mod pub_ns;

#[derive(Parser)]
pub enum SubCommand {
	/// Creates a development token
	///
	/// https://rivet.gg/docs/general/concepts/token-types#namespace-development
	#[clap(alias = "dev")]
	Development(dev::Opts),

	/// Creates a public namespace token
	///
	/// https://rivet.gg/docs/general/concepts/token-types#namespace-development
	#[clap(alias = "public", alias = "pub")]
	PublicNamespace(pub_ns::Opts),
}

impl SubCommand {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> Result<()> {
		match self {
			SubCommand::Development(opts) => opts.execute(ctx).await,
			SubCommand::PublicNamespace(opts) => opts.execute(ctx).await,
		}
	}
}
