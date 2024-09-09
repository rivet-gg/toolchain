use clap::Parser;
use serde::Serialize;
use std::process::ExitCode;
use toolchain::backend::run_opengb_command_passthrough;

/// Create a new module
#[derive(Parser, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Opts {
	#[clap(index = 1)]
	pub module: String,
}

impl Opts {
	pub async fn execute(&self) -> ExitCode {
		run_opengb_command_passthrough("createModule", self).await
	}
}
