use clap::Parser;
use serde::Serialize;
use std::process::ExitCode;
use toolchain::backend::run_opengb_command_passthrough;

/// Apply pre-generated migrations to a module
#[derive(Parser, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Opts {
	#[clap(long)]
	pub modules: Vec<String>,
}

impl Opts {
	pub async fn execute(&self) -> ExitCode {
		run_opengb_command_passthrough("dbMigrateApply", self).await
	}
}
