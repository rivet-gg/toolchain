use clap::Parser;
use serde::Serialize;
use std::process::ExitCode;
use toolchain::backend::run_opengb_command_passthrough;

/// Create a new script for a module
#[derive(Parser, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Opts {
	#[clap(long)]
	pub module: String,
	#[clap(long)]
	pub script: String,
}

impl Opts {
	pub async fn execute(&self) -> ExitCode {
		run_opengb_command_passthrough("createScript", self).await
	}
}

