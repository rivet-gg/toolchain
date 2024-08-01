use clap::Parser;

use global_error::prelude::*;
use serde::Serialize;

use super::SideKickHandler;

#[derive(Parser)]
pub struct Opts {}

#[derive(Serialize)]
pub struct Output {
	pub version: String,
}

impl SideKickHandler for Output {}

impl Opts {
	pub async fn execute(&self) -> GlobalResult<Output> {
		Ok(Output {
			version: concat!("v", env!("VERGEN_BUILD_SEMVER")).to_string(),
		})
	}
}
