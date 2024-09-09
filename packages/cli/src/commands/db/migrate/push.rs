use clap::Parser;
use serde::Serialize;
use std::process::ExitCode;
use toolchain::backend::run_opengb_command_passthrough;

/// Push a schema to the database without migrations
#[derive(Parser, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Opts {
	pub modules: Vec<String>,
}

impl Opts {
	pub async fn execute(&self) -> ExitCode {
		run_opengb_command_passthrough("dbMigratePush", self).await
	}
}
