use clap::Parser;
use serde::Serialize;
use std::process::ExitCode;
use toolchain::backend::run_opengb_command_passthrough;

/// Create a new script for a module
#[derive(Parser, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Opts {
	#[clap(index = 1)]
	pub module: String,
	#[clap(index = 2)]
	pub script: String,
}

impl Opts {
	pub async fn execute(&self) -> ExitCode {
		run_opengb_command_passthrough("createScript", self).await
	}
}
