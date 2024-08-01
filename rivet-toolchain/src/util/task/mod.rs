mod ctx;
mod log;

use global_error::prelude::*;
use serde::Deserialize;
use std::path::Path;
use tokio::{
	fs::OpenOptions,
	sync::{broadcast, mpsc},
	task,
	time::Duration,
};

pub use ctx::TaskCtx;

use crate::tasks::Task;

#[derive(Deserialize)]
pub struct RunConfig {
	/// Path to file that will abort this task if exists.
	pub abort_path: String,
	/// Path to file to output events.
	pub output_path: String,
}

/// Executes a future that can be aborted by touching a file.
pub async fn run_task<T>(run_config: RunConfig, input: T::Input) -> GlobalResult<T::Output>
where
	T: Task,
{
	let (log_tx, log_rx) = mpsc::unbounded_channel::<log::LogEvent>();
	let (shutdown_tx, shutdown_rx) = broadcast::channel(1);

	let output_file = OpenOptions::new()
		.create(true)
		.append(true)
		.open(run_config.output_path)
		.await?;

	task::spawn(log::log_writer(log_rx, output_file));

	let task_ctx = ctx::TaskCtxInner::new(log_tx, shutdown_rx);

	// Wait for task or abort
	let output = tokio::select! {
		result = T::run(task_ctx.clone(), input) => result,
		_ = wait_for_abort(&run_config.abort_path) => {
			Err(err_code!(ERROR, error = "Task aborted"))
		},
	};

	// Shutdown
	shutdown_tx.send(())?;

	output
}

const POLL_ABORT_INTERVAL: Duration = Duration::from_millis(250);

/// Waits for file to exist before completing.
async fn wait_for_abort(path: &str) {
	// HACK: Use file watcher
	let path = Path::new(path);
	loop {
		// TODO: Do this async
		if path.exists() {
			return;
		}
		tokio::time::sleep(POLL_ABORT_INTERVAL).await;
	}
}
