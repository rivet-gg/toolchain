use clap::Parser;

use global_error::prelude::*;
use serde::Serialize;

use super::SideKickHandler;

#[derive(Parser)]
pub struct Opts {}

#[derive(Serialize)]
pub struct Output {
	pub token: String,
}

impl SideKickHandler for Output {}

impl Opts {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> GlobalResult<Output> {
		Ok(Output {
			token: ctx.access_token.clone(),
		})
	}
}
