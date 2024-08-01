use clap::Parser;

use global_error::prelude::*;
use serde::Serialize;

use super::SideKickHandler;

#[derive(Parser)]
pub struct Opts {}

#[derive(Serialize)]
pub struct Output {
	pub token: String,
	pub api_endpoint: String,
	pub game_id: String,
}

impl SideKickHandler for Output {}

impl Opts {
	pub async fn execute(&self, ctx: &toolchain_core::Ctx) -> GlobalResult<Output> {
		Ok(Output {
			token: ctx.access_token.clone(),
			api_endpoint: ctx.api_endpoint.clone(),
			game_id: ctx.game_id.clone(),
		})
	}
}
