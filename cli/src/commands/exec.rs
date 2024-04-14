use crate::util::{cmd, term};
use clap::Parser;
use global_error::prelude::*;

#[derive(Parser)]
pub struct Opts {
	/// Script to run
	#[clap(index = 1)]
	pub command: String,

	/// Namespace to execute command against
	#[clap(short = 'n', long)]
	pub namespace: Option<String>,

	/// Test against your local machine to iterate quickly
	#[clap(short = 'l', alias = "local", alias = "this-machine", long)]
	pub dev: bool,

	/// Test against Rivet servers
	#[clap(short = 'r', alias = "remote", alias = "rivet-servers", long)]
	pub server: bool,
}

impl Opts {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> GlobalResult<()> {
		term::status::warn(
			"EXPERIMENTAL",
			"`rivet exec` is experimental and subject to change",
		);

		// Determine token
		let token = match (self.dev, self.server) {
			(true, true) => {
				bail!("Cannot use both --dev and --server");
			}
			(_, false) => cmd::RunWithRivetToken::ThisMachine,
			(_, true) => cmd::RunWithRivetToken::RivetServers,
		};

		// Run command
		cmd::run_with_rivet(
			ctx,
			cmd::RunWithRivetOpts {
				command: &self.command,
				env: Vec::new(),
				namespace: self.namespace.as_deref(),
				token,
			},
		)
		.await?;

		Ok(())
	}
}
