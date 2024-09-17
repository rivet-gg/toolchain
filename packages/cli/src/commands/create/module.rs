use clap::Parser;
use serde::Serialize;
use std::process::ExitCode;
use toolchain::backend::run_backend_command_passthrough;

use crate::util::global_opts::GlobalOpts;

/// Create a new module
#[derive(Parser, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Opts {
	#[clap(flatten)]
	#[serde(flatten)]
	global: GlobalOpts,

	#[clap(index = 1)]
	pub module: String,
}

impl Opts {
	pub async fn execute(&self) -> ExitCode {
		run_backend_command_passthrough("createModule", self).await
	}
}
