use clap::Parser;
use global_error::prelude::*;
use serde::Serialize;
use std::collections::HashMap;

use crate::commands::backend;

use super::SideKickHandler;

#[derive(Parser)]
pub struct Opts {
	#[clap(long)]
	no_color: bool,
	#[clap(long)]
	capture_output: bool,
}

#[derive(Serialize)]
pub struct Output {
	pub exit_code: i32,
	pub stdout: String,
	pub stderr: String,
}

impl SideKickHandler for Output {}

impl Opts {
	pub async fn execute(&self) -> GlobalResult<Output> {
		// Run command
		//
		// Force-deploy migrations since we don't have TTY access to prompt for
		// migrations. We also don't want to promp for migrations if the dev is
		// setting this up for the first time.
		let args = vec!["dev".into(), "--force-deploy-migrations".into()];
		let mut env = HashMap::new();
		if self.no_color {
			env.insert("OPENGB_TERM_COLOR".into(), "never".into());
		}
		let mut opengb_command = backend::build_opengb_command(backend::OpenGbCommandOpts {
			args,
			env,
			cwd: std::env::current_dir()?,
		})?;

		if self.capture_output {
			let opengb_output = opengb_command.output().await?;

			let output = Output {
				exit_code: opengb_output.status.code().unwrap_or(1),
				stdout: String::from_utf8(opengb_output.stdout)?,
				stderr: String::from_utf8(opengb_output.stderr)?,
			};

			Ok(output)
		} else {
			let opengb_status = opengb_command.status().await?;

			let output = Output {
				exit_code: opengb_status.code().unwrap_or(1),
				stdout: String::new(),
				stderr: String::new(),
			};

			Ok(output)
		}
	}
}
