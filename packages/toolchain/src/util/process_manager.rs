use anyhow::*;
use std::{collections::VecDeque, future::Future, process::Stdio, sync::Arc, time::Duration};
use tokio::{
	io::{AsyncBufReadExt, BufReader},
	process::Command,
	sync::{broadcast, mpsc, watch, Mutex},
};

use crate::util::task::TaskCtx;

const MAX_LOG_HISTORY: usize = 512;

#[derive(Debug, Clone)]
enum ProcessStatus {
	/// This procsss has not been started yet.
	NotRunning,

	/// Process is starting but the PID has not been determined yet.
	Starting,

	/// Currently running.
	#[allow(dead_code)]
	Running { pid: u32 },

	/// Currently stopping.
	Stopping,

	/// Process exited.
	Exited {
		exit_code: Option<i32>,
		error: Option<String>,
	},
}

impl ProcessStatus {
	fn is_running(&self) -> bool {
		matches!(
			self,
			ProcessStatus::Starting | ProcessStatus::Running { .. } | ProcessStatus::Stopping
		)
	}
}

#[derive(Debug, Clone)]
enum ProcessEvent {
	Log(ProcessLog),
}

#[derive(Debug, Clone)]
enum ProcessLog {
	Stdout(String),
	Stderr(String),
}

pub struct CommandOpts {
	pub command: String,
	pub args: Vec<String>,
	pub envs: Vec<(String, String)>,
	pub current_dir: String,
}

/// Manages the state of a process that's detached from the parent.
///
/// Allows for processes to stay running even if the engine restarts.
pub struct ProcessManager {
	pub key: &'static str,
	pub kill_grace: Duration,

	/// Sends a stop request to the process.
	stop_tx: Mutex<Option<mpsc::Sender<()>>>,

	/// Watch process status.
	status_tx: watch::Sender<ProcessStatus>,
	status_rx: watch::Receiver<ProcessStatus>,

	/// Broadcast process events.
	event_tx: broadcast::Sender<ProcessEvent>,

	/// Hold unused broadcast receiver so the sender does cancel.
	///
	/// All receivers will use `event_tx.subscribe()`
	_event_rx: broadcast::Receiver<ProcessEvent>,

	/// History of logs.
	logs: Mutex<VecDeque<ProcessLog>>,
}

impl ProcessManager {
	pub fn new(key: &'static str, kill_grace: Duration) -> Arc<Self> {
		let (status_tx, status_rx) = watch::channel(ProcessStatus::NotRunning);
		let (event_tx, event_rx) = broadcast::channel(16);
		Arc::new(Self {
			key,
			kill_grace,
			stop_tx: Mutex::new(None),
			status_tx,
			status_rx,
			event_tx,
			_event_rx: event_rx,
			logs: Mutex::new(VecDeque::new()),
		})
	}

	pub async fn start<CommandFut>(
		self: &Arc<Self>,
		task_ctx: TaskCtx,
		command_builder: impl FnOnce() -> CommandFut,
	) -> Result<Option<i32>>
	where
		CommandFut: Future<Output = Result<CommandOpts>>,
	{
		// Start new process if needed. Otherwise, will hook to the existing process.
		//
		// Clonign value required since this holds a read lock on the inner
		// value.
		if !self.status_rx.borrow().is_running() {
			// Build command
			//
			// Do this before assigning process id in case the builder fails
			let command_opts = command_builder().await?;

			// Spawn process
			self.spawn_process(command_opts).await?;
		};

		// Write events to task
		let mut event_rx = self.event_tx.subscribe();
		let log_fut = async {
			// Write all log history
			{
				let logs = self.logs.lock().await;
				for line in logs.iter().rev() {
					match line {
						ProcessLog::Stdout(line) => {
							task_ctx.log(format!("[stdout] {line}"));
						}
						ProcessLog::Stderr(line) => {
							task_ctx.log(format!("[stderr] {line}"));
						}
					}
				}
			}

			// Wait for events
			while let Result::Ok(event) = event_rx.recv().await {
				match event {
					ProcessEvent::Log(ProcessLog::Stdout(line)) => {
						task_ctx.log(format!("[stdout] {line}"));
					}
					ProcessEvent::Log(ProcessLog::Stderr(line)) => {
						task_ctx.log(format!("[stderr] {line}"));
					}
				}
			}
		};

		// Wait for process to exit
		let mut status_rx = self.status_tx.subscribe();
		tokio::select! {
			res = status_rx.wait_for(|x| matches!(x, ProcessStatus::Exited { .. })) => {
				// Destructure exit
				let ProcessStatus::Exited { exit_code, error } = res.context("wait for exit")?.clone() else {
					bail!("unreachable");
				};

				// Re-throw error
				if let Some(error) = error {
					bail!("process erorr: {error}");
				}

				Ok(exit_code)
			}
			_ = log_fut => {
				bail!("log fut exited early");
			}
		}
	}

	pub async fn stop(&self) -> Result<bool> {
		// Clonign value required since this holds a read lock on the inner
		// value.
		if matches!(
			*self.status_rx.borrow(),
			ProcessStatus::Running { .. } | ProcessStatus::Starting
		) {
			let mut status_rx = self.status_tx.subscribe();
			let mut stop = self.stop_tx.lock().await;

			// Stop can only be sent once, so take the sender
			if let Some(stop_tx) = stop.take() {
				stop_tx.send(()).await?;

				// Wait for stop
				status_rx
					.wait_for(|x| matches!(x, ProcessStatus::Exited { .. }))
					.await?;

				Ok(true)
			} else {
				Ok(false)
			}
		} else {
			Ok(false)
		}
	}

	pub async fn is_running(&self) -> Result<bool> {
		Ok(self.status_rx.borrow().is_running())
	}

	async fn spawn_process(self: &Arc<Self>, command_opts: CommandOpts) -> Result<()> {
		// Create new shutdown channel
		let (stop_tx, stop_rx) = mpsc::channel(1);
		*self.stop_tx.lock().await = Some(stop_tx);

		// Update status
		self.status_tx.send(ProcessStatus::Starting)?;

		// Run inner and catch state
		let _self = self.clone();
		tokio::spawn(async move {
			match _self.spawn_process_inner(command_opts, stop_rx).await {
				Result::Ok(_) => {}
				Err(err) => {
					let _ = _self.status_tx.send(ProcessStatus::Exited {
						exit_code: None,
						error: Some(err.to_string()),
					});
					let _ = _self.clear_logs().await;
				}
			}
		});

		Ok(())
	}

	async fn spawn_process_inner(
		self: &Arc<Self>,
		command_opts: CommandOpts,
		mut stop_rx: mpsc::Receiver<()>,
	) -> Result<()> {
		let mut cmd = Command::new(command_opts.command);
		cmd.current_dir(command_opts.current_dir)
			.args(command_opts.args)
			.envs(command_opts.envs.iter().cloned());

		// Required in case this task is cancelled
		cmd.kill_on_drop(true);

		// Configure the command to pipe stdout and stderr
		cmd.stdout(Stdio::piped());
		cmd.stderr(Stdio::piped());

		#[cfg(windows)]
		{
			use windows::Win32::System::Threading::CREATE_NEW_PROCESS_GROUP;
			cmd.creation_flags(CREATE_NEW_PROCESS_GROUP.0);
		}

		// Spawn the command
		let mut child = cmd.spawn()?;

		// Update state
		let child_pid = child.id().expect("missing child pid");
		self.status_tx
			.send(ProcessStatus::Running { pid: child_pid })
			.context("send ProcessStatus::Running")?;

		// Setup log handlers
		let stdout = child.stdout.take().expect("Failed to capture stdout");
		let stderr = child.stderr.take().expect("Failed to capture stderr");

		let mut stdout_reader = BufReader::new(stdout).lines();
		let mut stderr_reader = BufReader::new(stderr).lines();

		// Spawn tasks to handle stdout and stderr
		let _self = self.clone();
		let stdout_handle = tokio::spawn(async move {
			while let Result::Ok(Some(line)) = stdout_reader.next_line().await {
				_self.add_log(ProcessLog::Stdout(line)).await?;
			}

			Result::<()>::Ok(())
		});

		let _self = self.clone();
		let stderr_handle = tokio::spawn(async move {
			while let Result::Ok(Some(line)) = stderr_reader.next_line().await {
				_self.add_log(ProcessLog::Stderr(line)).await?;
			}

			Result::<()>::Ok(())
		});

		// Wait for process
		let exit_code = tokio::select! {
			res = child.wait() => {
				let status = res?;
				status.code()
			}
			res = stop_rx.recv() => {
				res.context("stop_rx.recv")?;

				// Update state
				self.status_tx.send(ProcessStatus::Stopping).context("send ProcessStatus::Stopping")?;

				// Send SIGTERM to stop gracefully
				self.send_terminate_signal(child_pid).await?;

				// Wait for process to exit
				match tokio::time::timeout(self.kill_grace, child.wait()).await {
					Result::Ok(Result::Ok(status)) => {
						// Stopped gracefully
						status.code()
					}
					Result::Ok(Err(err)) => {
						// Error waiting for process
						return Err(err.into());
					}
					Err(_) => {
						// Timed out, force kill
						child.kill().await.context("kill failed")?;

						None
					}
				}
			}
		};

		// Wait for log handles in order to not miss any logs
		stdout_handle.await??;
		stderr_handle.await??;

		// Update state
		self.status_tx
			.send(ProcessStatus::Exited {
				exit_code,
				error: None,
			})
			.context("send ProcessStatus::Exited")?;
		self.clear_logs().await?;

		Ok(())
	}

	async fn add_log(&self, log: ProcessLog) -> Result<()> {
		// Write log
		{
			let mut logs = self.logs.lock().await;
			logs.push_front(log.clone());
			logs.truncate(MAX_LOG_HISTORY);
		}

		// Publish event
		self.event_tx.send(ProcessEvent::Log(log))?;

		Ok(())
	}

	async fn clear_logs(&self) -> Result<()> {
		let mut logs = self.logs.lock().await;
		logs.clear();
		Ok(())
	}

	async fn send_terminate_signal(&self, pid: u32) -> Result<()> {
		#[cfg(unix)]
		{
			use nix::sys::signal::{kill, Signal};
			use nix::unistd::Pid;

			kill(Pid::from_raw(pid as i32), Signal::SIGTERM)?;
		}

		#[cfg(windows)]
		{
			use windows::Win32::System::Console::{GenerateConsoleCtrlEvent, CTRL_BREAK_EVENT};

			unsafe {
				// Attempt to terminate the process gracefully
				if !GenerateConsoleCtrlEvent(CTRL_BREAK_EVENT, pid as u32).as_bool() {
					bail!("failed to terminate process")
				}
			}
		}

		Ok(())
	}
}
