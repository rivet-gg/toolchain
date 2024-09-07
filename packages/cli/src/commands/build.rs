use clap::Parser;
use serde::Serialize;
use std::process::ExitCode;
use toolchain::backend::run_opengb_command_passthrough;

#[derive(Parser, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Opts {
	#[clap(long)]
	pub watch: bool,
	#[clap(long)]
	pub runtime: Option<String>,
	#[clap(long)]
	pub output_format: Option<String>,
	#[clap(long)]
	pub db_driver: Option<String>,
	#[clap(long)]
	pub migrate: bool,
	#[clap(long)]
	pub migrate_mode: Option<String>,
	#[clap(long)]
	pub strict_schemas: bool,
}

impl Opts {
	pub async fn execute(&self) -> ExitCode {
		run_opengb_command_passthrough("build", self).await
	}
}
