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
	#[clap(short = 'l', alias = "local", long)]
	pub this_machine: bool,

	/// Test against Rivet servers
	#[clap(short = 'r', alias = "remote", long)]
	pub rivet_servers: bool,
}

impl Opts {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> GlobalResult<()> {
		term::status::warn(
			"EXPERIMENTAL",
			"`rivet exec` is experimental and subject to change",
		);

		// Determine token
		let token = match (self.this_machine, self.rivet_servers) {
			(true, false) | (false, false) => cmd::RunWithRivetToken::ThisMachine,
			(false, true) => cmd::RunWithRivetToken::ThisMachine,
			_ => {
				bail!("Cannot use both --this-machine and --rivet-servers");
			}
		};

		// Run command
		cmd::run_with_rivet(
			ctx,
			cmd::RunWithRivetOpts {
				command: &self.command,
				envs: Vec::new(),
				namespace: self.namespace.as_deref(),
				token,
			},
		)
		.await?;

		Ok(())
	}
}
