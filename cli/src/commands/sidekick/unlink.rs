use clap::Parser;
use global_error::prelude::*;
use serde::Serialize;

use crate::commands;

use super::SideKickHandler;

#[derive(Parser)]
pub struct Opts {}

#[derive(Serialize)]
pub struct Output {}

impl SideKickHandler for Output {}

impl Opts {
	pub async fn execute(&self, _ctx: &cli_core::Ctx) -> GlobalResult<Output> {
		commands::unlink::unlink().await?;

		Ok(Output {})
	}
}
