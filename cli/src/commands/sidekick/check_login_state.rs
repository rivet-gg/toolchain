use clap::Parser;

use global_error::prelude::*;
use serde::Serialize;

use super::SideKickHandler;

#[derive(Parser)]
pub struct Opts {}

#[derive(Serialize)]
pub struct Output {}

impl SideKickHandler for Output {}

impl Opts {
	pub async fn execute(&self) -> GlobalResult<Output> {
		Ok(Output {})
	}
}
