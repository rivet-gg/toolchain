use clap::Parser;
use serde::Serialize;
use std::process::ExitCode;

use crate::util::global_opts::GlobalOpts;

/// Initiate a new project
#[derive(Parser, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Opts {
	#[clap(flatten)]
	#[serde(flatten)]
	global: GlobalOpts,

	#[clap(long, default_value = ".")]
	pub dir: String,
}

impl Opts {
	pub async fn execute(&self) -> ExitCode {
		todo!()
	}
}
