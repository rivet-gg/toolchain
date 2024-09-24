use clap::Parser;
use serde::Serialize;
use std::process::ExitCode;
use toolchain::backend::run_backend_command_passthrough;

use crate::util::{global_opts::GlobalOpts, postgres};

/// Compile the backend for self-hosting
#[derive(Parser, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Opts {
	#[clap(flatten)]
	#[serde(flatten)]
	global: GlobalOpts,

	#[clap(long, default_value = "false")]
	pub watch: bool,
	#[clap(long, default_value = "deno")]
	pub runtime: Option<String>,
	#[clap(long, default_value = "native")]
	pub output_format: String,
	#[clap(long, default_value = "node_postgres")]
	pub db_driver: String,
	#[clap(long, default_value = "true")]
	pub sdk: bool,
	#[clap(long, default_value = "true")]
	pub migrate: bool,
	#[clap(long, default_value = "generate")]
	pub migrate_mode: Option<String>,
	#[clap(long, default_value = "true")]
	pub strict_schemas: bool,
}

impl Opts {
	pub async fn execute(&self) -> ExitCode {
		let Ok(_) = postgres::ensure_running().await else {
			return ExitCode::FAILURE;
		};

		run_backend_command_passthrough("build.ts", self).await
	}
}
