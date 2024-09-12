use clap::Parser;
use serde::Serialize;
use std::process::ExitCode;
use toolchain::backend::run_backend_command_passthrough;

/// Initiate a new project
#[derive(Parser, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Opts {
	#[clap(long, default_value = ".")]
	pub dir: String,
}

impl Opts {
	pub async fn execute(&self) -> ExitCode {
		run_backend_command_passthrough("init", self).await
	}
}
