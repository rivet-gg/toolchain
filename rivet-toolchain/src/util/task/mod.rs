mod ctx;
pub mod output;

use global_error::prelude::*;
use serde::Deserialize;
use std::path::Path;
use tokio::{
	sync::{broadcast, mpsc},
	task,
	time::Duration,
};

pub use ctx::TaskCtx;
pub use output::OutputEvent;

use crate::tasks::Task;

#[derive(Deserialize, Clone, Default)]
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

	/// How output is printed.
	#[serde(default)]
	pub output_style: OutputStyle,
}

#[derive(Deserialize, Copy, Clone)]
pub enum OutputStyle {
	Json,
	Plain,
}

impl Default for OutputStyle {
	fn default() -> Self {
		Self::Json
	}
}

/// Executes a future that can be aborted by touching a file.
pub async fn run_task<T>(run_config: RunConfig, input: T::Input) -> GlobalResult<T::Output>
where
	T: Task,
{
	let (output_tx, output_rx) = mpsc::unbounded_channel::<output::OutputEvent>();
	let (shutdown_tx, shutdown_rx) = broadcast::channel(1);

	// Open output file
	let output_file = if let Some(output_path) = run_config.output_path {
		Some(tokio::task::block_in_place(|| {
			std::fs::OpenOptions::new()
				.create(true)
				.append(true)
				.open(output_path)
		})?)
	} else {
		None
	};

	// Start writer
	let output_writer_task = task::spawn(output::output_writer(
		output_rx,
		output_file,
		run_config.output_style,
	));

	// Create context
	let task_ctx = ctx::TaskCtxInner::new(output_tx, shutdown_rx);

	// RUn task or wait for abort
	let output = tokio::select! {
		result = T::run(task_ctx.clone(), input) => result,
		_ = wait_for_abort(run_config.abort_path.clone()) => {
			Err(err_code!(ERROR, error = "Task aborted"))
		},
	};

	// Write output to log
	task_ctx.log_output(&output)?;

	// Shutdown
	shutdown_tx.send(())?;

	// Drop context since this holds a ref to output_tx
	drop(task_ctx);

	// Wait for task to finish
	match output_writer_task.await {
		Ok(_) => {}
		Err(err) => {
			eprintln!("Task failed: {err:?}");
		}
	}

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
