mod ctx;
mod log;

use global_error::prelude::*;
use serde::Deserialize;
use std::path::Path;
use tempfile::TempDir;
use tokio::{
	fs::OpenOptions,
	sync::{broadcast, mpsc},
	task,
	time::Duration,
};

pub use ctx::TaskCtx;

use crate::tasks::Task;

#[derive(Deserialize, Clone)]
pub struct RunConfig {
	/// Path to file that will abort this task if exists.
	///
	/// If none provided, the task will not be abortable.
	#[serde(default)]
	pub abort_path: Option<String>,

	/// Path to file to output events.
	///
	/// If none provided, will be logged to standard output.
	#[serde(default)]
	pub output_path: Option<String>,
}

impl RunConfig {
	pub fn empty() -> Self {
		RunConfig {
			abort_path: None,
			output_path: None,
		}
	}

	/// Creates a new config with paths in a temp dir.
	pub fn with_temp_dir() -> GlobalResult<(Self, TempDir)> {
		let temp_dir = tempfile::tempdir()?;

		Ok((
			Self {
				abort_path: Some(temp_dir.path().join("abort").display().to_string()),
				output_path: Some(temp_dir.path().join("output").display().to_string()),
			},
			temp_dir,
		))
	}
}

/// Executes a future that can be aborted by touching a file.
pub async fn run_task<T>(run_config: RunConfig, input: T::Input) -> GlobalResult<T::Output>
where
	T: Task,
{
	let (log_tx, log_rx) = mpsc::unbounded_channel::<log::LogEvent>();
	let (shutdown_tx, shutdown_rx) = broadcast::channel(1);

	let output_file = if let Some(output_path) = run_config.output_path {
		Some(
			OpenOptions::new()
				.create(true)
				.append(true)
				.open(output_path)
				.await?,
		)
	} else {
		None
	};

	task::spawn(log::log_writer(log_rx, output_file));

	let task_ctx = ctx::TaskCtxInner::new(log_tx, shutdown_rx);

	// Wait for task or abort
	let output = tokio::select! {
		result = T::run(task_ctx.clone(), input) => result,
		_ = wait_for_abort(run_config.abort_path.clone()) => {
			Err(err_code!(ERROR, error = "Task aborted"))
		},
	};

	// Shutdown
	shutdown_tx.send(())?;

	output
}

const POLL_ABORT_INTERVAL: Duration = Duration::from_millis(250);

/// Waits for file to exist before completing.
async fn wait_for_abort(path: Option<String>) {
	let Some(path) = path else {
		// Wait forever, since this task is not abortable.
		return std::future::pending::<()>().await;
	};

	// HACK: Use file watcher
	let path = Path::new(&path);
	loop {
		// TODO: Do this async
		if path.exists() {
			return;
		}
		tokio::time::sleep(POLL_ABORT_INTERVAL).await;
	}
}
