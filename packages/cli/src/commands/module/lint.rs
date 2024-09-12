use clap::Parser;
use serde::Serialize;
use std::process::ExitCode;
use toolchain::backend::run_backend_command_passthrough;

#[derive(Parser, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Opts {}

impl Opts {
	pub async fn execute(&self) -> ExitCode {
		run_backend_command_passthrough("lint", self).await
	}
}
