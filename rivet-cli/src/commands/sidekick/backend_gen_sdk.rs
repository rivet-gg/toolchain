use clap::Parser;
use global_error::prelude::*;
use serde::Serialize;
use std::collections::HashMap;

use crate::commands::backend;

use super::SideKickHandler;

#[derive(Parser)]
pub struct Opts {
	#[clap(long)]
	output_path: String,
	#[clap(long)]
	unity: bool,
	#[clap(long)]
	godot: bool,
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
		let target = if self.unity {
			"unity"
		} else if self.godot {
			"godot"
		} else {
			bail!("no target selected")
		};
		let args = vec![
			"sdk".into(),
			"generate".into(),
			"--output".into(),
			self.output_path.clone(),
			target.into(),
		];
		let mut env = HashMap::new();
		env.insert("OPENGB_TERM_COLOR".into(), "never".into());
		let opengb_output = backend::build_opengb_command(backend::OpenGbCommandOpts {
			args,
			env,
			cwd: std::env::current_dir()?,
		})?
		.output()
		.await?;

		let output = Output {
			exit_code: opengb_output.status.code().unwrap_or(1),
			stdout: String::from_utf8(opengb_output.stdout)?,
			stderr: String::from_utf8(opengb_output.stderr)?,
		};

		Ok(output)
	}
}
