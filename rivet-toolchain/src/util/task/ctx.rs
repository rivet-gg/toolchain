use global_error::prelude::*;
use std::{process::Stdio, sync::Arc};
use tokio::{
	io::{AsyncBufReadExt, BufReader},
	process::Command,
	sync::{broadcast, mpsc},
};

use super::log::LogEvent;

// HACK: Tokio bug drops the channel using the native `UnboundedSender::clone`, so we have to use
// `Arc`.
pub type TaskCtx = Arc<TaskCtxInner>;

/// Logs to arbitrary files asynchronously.
///
/// Allows us to store separate log files for different tasks that are running in a headless
/// context outside of a CLI.
pub struct TaskCtxInner {
	log_tx: mpsc::UnboundedSender<LogEvent>,
	#[allow(dead_code)]
	shutdown_rx: broadcast::Receiver<()>,
}

impl TaskCtxInner {
	pub fn new(
		log_tx: mpsc::UnboundedSender<LogEvent>,
		shutdown_rx: broadcast::Receiver<()>,
	) -> Arc<Self> {
		Arc::new(Self {
			log_tx,
			shutdown_rx,
		})
	}

	pub fn log_stdout(&self, message: impl ToString) {
		let _ = self.log_tx.send(LogEvent::Stdout(message.to_string()));
	}

	pub fn log_stderr(&self, message: impl ToString) {
		let _ = self.log_tx.send(LogEvent::Stderr(message.to_string()));
	}

	pub async fn spawn_cmd(
		self: &Arc<Self>,
		mut cmd: Command,
	) -> GlobalResult<std::process::ExitStatus> {
		// Required in case this task is cancelled
		cmd.kill_on_drop(true);

		// Configure the command to pipe stdout and stderr
		cmd.stdout(Stdio::piped());
		cmd.stderr(Stdio::piped());

		// Spawn the command
		let mut child = cmd.spawn()?;

		// Take ownership of the stdout and stderr handles
		let stdout = child.stdout.take().expect("Failed to capture stdout");
		let stderr = child.stderr.take().expect("Failed to capture stderr");

		// Create buffered readers
		let mut stdout_reader = BufReader::new(stdout).lines();
		let mut stderr_reader = BufReader::new(stderr).lines();

		// Clone the logger for use in the spawned tasks
		let stdout_logger = self.clone();
		let stderr_logger = self.clone();

		// Spawn tasks to handle stdout and stderr
		tokio::spawn(async move {
			while let Ok(Some(line)) = stdout_reader.next_line().await {
				stdout_logger.log_stdout(line);
			}
		});

		tokio::spawn(async move {
			while let Ok(Some(line)) = stderr_reader.next_line().await {
				stderr_logger.log_stderr(line);
			}
		});

		// Wait for the command to finish and get its exit status
		let status = child.wait().await?;

		Ok(status)
	}
}
