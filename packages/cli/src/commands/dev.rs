use clap::Parser;
use serde::Serialize;
use std::process::ExitCode;
use toolchain::{
	backend::{run_opengb_command_passthrough, BackendCommandOpts},
};

/// Run the development server
#[derive(Parser, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Opts {
	#[arg(long, default_value = "true")]
	build: bool,
	#[arg(long, default_value = "true")]
	check: bool,
	#[arg(long, default_value = "true")]
	strict_schemas: bool,
	#[arg(long, default_value = "true")]
	watch: bool,
	#[arg(long, default_value = "true")]
	migrate: bool,
	#[arg(long, default_value = "dev")]
	migrate_mode: String,
	#[arg(long, default_value = "false")]
	non_interactive: bool,
}

impl Opts {
	pub async fn execute(&self) -> ExitCode {
		run_opengb_command_passthrough(BackendCommandOpts {
			command: "dev",
			opts: match serde_json::to_value(self) {
				Ok(x) => x,
				Err(err) => {
					eprintln!("Failed to serialize body: {err:?}");
					return ExitCode::FAILURE;
				}
			},
			env: Default::default(),
		})
		.await
	}
}
