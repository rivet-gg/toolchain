use global_error::prelude::*;
use clap::Parser;

use crate::{
	commands,
	util::{cmd, global_config},
};

#[derive(Parser)]
pub struct Opts {
	#[clap(index = 1)]
	pub command: String,

	/// Namespace to create token for
	#[clap(short = 'n', long)]
	pub namespace: Option<String>,
}

impl Opts {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> GlobalResult<()> {
		let api_endpoint = unwrap!(global_config::read_project(|x| x.tokens.cloud.clone())
			.await?, "cloud token");

		// Get dev token
		let token = commands::token::create::dev::execute(
			ctx,
			&commands::token::create::dev::Opts {
				namespace: self.namespace.clone(),
			},
		)
		.await?;

		// Run command
		cmd::run_script(
			&self.command,
			vec![
				("RIVET_TOKEN".into(), token.token),
				("RIVET_API_ENDPOINT".into(), api_endpoint),
			],
		)
		.await?;

		Ok(())
	}
}
