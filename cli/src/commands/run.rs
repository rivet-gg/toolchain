use clap::Parser;
use global_error::prelude::*;

use crate::{
	commands,
	util::{cmd, term},
};

#[derive(Parser)]
pub struct Opts {
	/// Name of the script to run
	#[clap(index = 1)]
	pub script: String,

	/// Namespace to execute script against
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
			"`rivet run` is experimental and subject to change",
		);

		// Read script
		let config = crate::commands::config::read_config(
			Vec::new(),
			self.namespace.as_ref().map(String::as_str),
		)
		.await?;
		let command = unwrap_with!(
			config.scripts.as_ref().and_then(|x| x.get(&self.script)),
			CLI_SCRIPT_NOT_FOUND
		);

		// Get dev token
		let token = match (self.this_machine, self.rivet_servers) {
			(true, false) => {
				commands::token::create::dev::execute(
					ctx,
					&commands::token::create::dev::Opts {
						namespace: self.namespace.clone(),
					},
				)
				.await?
				.token
			}
			(false, true) => {
				commands::token::create::dev::execute(
					ctx,
					&commands::token::create::dev::Opts {
						namespace: self.namespace.clone(),
					},
				)
				.await?
				.token
			}
			_ => {
				bail!("Cannot use both --this-machine and --rivet-servers");
			}
		};

		// Run command
		cmd::run_script(
			&command,
			vec![
				("RIVET_API_ENDPOINT".into(), ctx.api_endpoint.clone()),
				("RIVET_TOKEN".into(), token),
			],
		)
		.await?;

		Ok(())
	}
}
