use clap::Parser;
use global_error::prelude::*;
use serde::Serialize;
use std::fs::File;
use tokio::{process::Command, signal};

use super::SideKickHandler;

#[derive(Parser)]
pub struct Opts {
	#[clap(long)]
	stdout: String,
	#[clap(long)]
	stderr: String,
	#[clap(index = 1)]
	cmd: String,
	#[clap(index = 2, multiple_values = true)]
	args: Vec<String>,
}

#[derive(Serialize)]
pub struct Output {}

impl SideKickHandler for Output {}

impl Opts {
	pub async fn execute(&self) -> GlobalResult<Output> {
		let mut signal = signal::ctrl_c();

		// Spawn process & pipe output to file
		let stdout_file = File::create(&self.stdout)?;
		let stderr_file = File::create(&self.stderr)?;
		let mut child = Command::new(&self.cmd)
			.args(&self.args)
			.stdout(stdout_file)
			.stderr(stderr_file)
			.kill_on_drop(true)
			.spawn()?;

		// Wait for process or kill with signal
		tokio::select! {
			result = child.wait() => {
				println!("Child finished: {result:?}");
			}
			_ = signal => {
				println!("Received shutdown signal, terminating child process...");
				if let Err(e) = child.kill().await {
					eprintln!("Failed to kill child process: {}", e);
				}
			}
		}

		Ok(Output {})
	}
}

