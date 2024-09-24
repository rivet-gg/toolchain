use clap::Parser;
use serde::Serialize;
use std::process::ExitCode;
use toolchain::backend::run_backend_command_passthrough;

use crate::util::global_opts::GlobalOpts;

#[derive(Parser, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Opts {
	#[clap(flatten)]
	#[serde(flatten)]
	global: GlobalOpts,

	pub modules_filter: Vec<String>,
	#[clap(long, default_value = "true")]
	pub build: bool,
	#[clap(long, default_value = "true")]
	pub check: bool,
	#[clap(long, default_value = "true")]
	pub strict_schemas: bool,
	#[clap(long, default_value = "true")]
	pub sdk: bool,
	#[clap(long, default_value = "true")]
	pub migrate: bool,
	#[clap(long, default_value = "dev")]
	pub migrate_mode: String,
	#[clap(long, default_value = "false")]
	pub watch: bool,
	#[clap(long)]
	pub filter: Option<String>,
}

impl Opts {
	pub async fn execute(&self) -> ExitCode {
		let Ok(_) = postgres::ensure_running().await else {
			return ExitCode::FAILURE;
		};

		run_backend_command_passthrough("test.ts", self).await
	}
}
