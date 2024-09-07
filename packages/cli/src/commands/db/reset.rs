use clap::Parser;
use serde::Serialize;
use std::process::ExitCode;
use toolchain::backend::run_opengb_command_passthrough;

/// Delete all data from database
#[derive(Parser, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Opts {
	#[clap(long, default_value = "[]")]
	pub modules: Vec<String>,
}

impl Opts {
pub async fn execute(&self) -> ExitCode {
	run_opengb_command_passthrough("dbReset", self).await
}


}