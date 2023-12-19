use clap::Parser;
use global_error::prelude::*;
use serde::Serialize;

use crate::commands;

use super::SideKickHandler;

#[derive(Parser)]
pub struct Opts {
	#[clap(long)]
	namespace: String,

	#[clap(long)]
	remote: bool,
}

#[derive(Serialize)]
pub struct Output {
	pub token: String,
}

impl SideKickHandler for Output {}

impl Opts {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> GlobalResult<Output> {
		let token = commands::token::create::dev::execute(
			ctx,
			&commands::token::create::dev::Opts {
				namespace: Some(self.namespace.clone()),
			},
		)
		.await?
		.token;

		Ok(Output { token })
	}
}
