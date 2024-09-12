use anyhow::*;
use rivet_process_supervisor_shared as shared;
use serde::{Deserialize, Serialize};
use std::{
	future::Future,
	path::PathBuf,
	process::{Command, Stdio},
	time::{Duration, Instant},
};
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use uuid::Uuid;

use crate::{config, util::task::TaskCtx};

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum StartMode {
	StartOrHook,
	HookOnly,
}

pub struct StartOpts {
	pub task: TaskCtx,
	pub base_data_dir: PathBuf,
	pub start_mode: StartMode,
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
#[derive(Clone)]
pub struct ProcessManager {
	pub key: &'static str,
	pub kill_grace: Duration,
}

impl ProcessManager {
	pub async fn start<CommandFut>(
		&self,
		StartOpts {
			task,
			base_data_dir,
			start_mode,
		}: StartOpts,
		command_builder: impl FnOnce() -> CommandFut,
	) -> Result<Option<i32>>
	where
		CommandFut: Future<Output = Result<CommandOpts>>,
	{
		// Check if existing process exists
		//
		// Preserving the process ID in settings serves a few purposes:
		// - Some game engines like Unity frequently restart the plugin, so the
		//   process need to run independently
		// - Game server process often hog a port, so we need to kill the
		//   previous process to ensure the port is free
		let process_id =
			if let Some(process_id) = self.read_state(&base_data_dir, |x| x.process_id).await? {
				// Check if process exists
				if matches!(
					get_process_state(process_id, &base_data_dir).await?,
					ProcessState::Running { .. }
				) {
					Some(process_id)
				} else {
					None
				}
			} else {
				None
			};

		// If process does not exist, spawn a new process
		let process_id = if let Some(process_id) = process_id {
			process_id
		} else {
			match start_mode {
				StartMode::StartOrHook => {
					// Build command
					//
					// Do this before assigning process id in case the builder fails
					let command_opts = command_builder().await?;

					// Set process ID before starting to prevent race condition
					let process_id = Uuid::new_v4();
					self.mutate_state(&base_data_dir, |meta| meta.process_id = Some(process_id))
						.await?;

					// Create data directory if it doesn't exist
					let process_data_dir = process_data_dir(process_id, &base_data_dir)?;
					std::fs::create_dir_all(&process_data_dir)?;

					// Spawn orphan
					let process_supervisor_path =
						rivet_process_supervisor_embed::get_executable(&base_data_dir)?;
					spawn_orphaned_process(
						process_supervisor_path,
						process_data_dir,
						&command_opts.current_dir,
						&command_opts.command,
						&command_opts
							.args
							.iter()
							.map(|s| s.as_str())
							.collect::<Vec<_>>(),
						&command_opts.envs,
					)?;

					process_id
				}
				StartMode::HookOnly => {
					// Don't start new process
					return Ok(None);
				}
			}
		};

		// Wait for process to exit
		let started_at = Instant::now();
		loop {
			let process_state = get_process_state(process_id, &base_data_dir).await?;
			match process_state {
				ProcessState::Starting => {
					// Process is still starting. The process supervisor will write the PID to the
					// thread once ready.
					//
					// This may time out if the command is not found since the process manager
					// cannot start.

					// Time out
					if started_at.elapsed() > Duration::from_secs(2) {
						bail!("timed out waiting for process tos tart");
					}

					// Wait for process to start
					tokio::time::sleep(Duration::from_millis(100)).await;

					continue;
				}
				ProcessState::Running { pid } => {
					let process_data_dir = process_data_dir(process_id, &base_data_dir)?;
					#[cfg(test)]
					eprintln!("Data dir: {}", process_data_dir.display());

					// Wait for process to exit & stream logs
					let stdout_path = process_data_dir.join(shared::paths::CHILD_STDOUT);
					let stderr_path = process_data_dir.join(shared::paths::CHILD_STDERR);
					tokio::select! {
						res = wait_pid_exit(pid) => {
							res?;
						}
						res = tail_logs(stdout_path, task.clone(), "stdout") => {
							res?;
							bail!("stdout logs exited early");
						}
						res = tail_logs(stderr_path, task.clone(), "stderr") => {
							res?;
							bail!("stderr logs exited early");
						}
					}

					// Read the exit code file asynchronously
					let exit_code_path = process_data_dir.join(shared::paths::CHILD_EXIT_CODE);
					let exit_code = match tokio::fs::read_to_string(&exit_code_path).await {
						Result::Ok(content) => match content.trim().parse::<i32>() {
							Result::Ok(code) => Some(code),
							Err(_) => None,
						},
						Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
						Err(e) => return Err(e.into()),
					};

					return Ok(exit_code);
				}
				ProcessState::Exited { exit_code } => {
					// Exited immediately

					return Ok(exit_code);
				}
				ProcessState::NotFound => {
					bail!("unexpected, process not found")
				}
			}
		}
	}

	pub async fn stop(&self, base_data_dir: &PathBuf) -> Result<bool> {
		let did_kill = if let Some(process_id) = self
			.mutate_state(base_data_dir, |x| x.process_id.take())
			.await?
		{
			kill_process(process_id, self.kill_grace, base_data_dir).await?
		} else {
			false
		};

		Ok(did_kill)
	}

	pub async fn is_running(&self, base_data_dir: &PathBuf) -> Result<bool> {
		if let Some(process_id) = self.read_state(base_data_dir, |x| x.process_id).await? {
			Ok(matches!(
				get_process_state(process_id, base_data_dir).await?,
				ProcessState::Starting | ProcessState::Running { .. }
			))
		} else {
			Ok(false)
		}
	}

	async fn read_state<F: FnOnce(&config::meta::ProcessState) -> T, T>(
		&self,
		base_data_dir: &PathBuf,
		cb: F,
	) -> Result<T> {
		config::meta::mutate_project(base_data_dir, |meta| {
			cb(meta.processes.entry(self.key.to_string()).or_default())
		})
		.await
	}

	async fn mutate_state<F: FnOnce(&mut config::meta::ProcessState) -> T, T>(
		&self,
		base_data_dir: &PathBuf,
		cb: F,
	) -> Result<T> {
		config::meta::mutate_project(base_data_dir, |meta| {
			cb(meta.processes.entry(self.key.to_string()).or_default())
		})
		.await
	}
}

/// Directory where the process state is stored.
fn process_data_dir(process_id: Uuid, base_data_dir: &PathBuf) -> Result<PathBuf> {
	let process_data_dir = base_data_dir.join("process").join(process_id.to_string());
	Ok(process_data_dir)
}

#[derive(Debug, Clone)]
enum ProcessState {
	/// Process is starting but the PID has not been determined yet.
	Starting,

	/// Currently running.
	Running { pid: i32 },

	/// Process exited.
	Exited { exit_code: Option<i32> },

	/// Process data dir does not exist.
	NotFound,
}

async fn get_process_state(process_id: Uuid, base_data_dir: &PathBuf) -> Result<ProcessState> {
	let process_data_dir = process_data_dir(process_id, base_data_dir)?;

	// Check if the data directory exists
	if !process_data_dir.exists() {
		return Ok(ProcessState::NotFound);
	}

	// Check if the exit_code file exists
	let exit_code_path = process_data_dir.join(shared::paths::CHILD_EXIT_CODE);
	if exit_code_path.exists() {
		let exit_code_str = tokio::fs::read_to_string(exit_code_path).await?;
		let exit_code: i32 = exit_code_str.trim().parse()?;
		return Ok(ProcessState::Exited {
			exit_code: Some(exit_code),
		});
	}

	// Check if the pid file exists
	let pid_path = process_data_dir.join(shared::paths::SUPERVISOR_PID);
	if pid_path.exists() {
		// Read the PID from the file
		let pid_str = tokio::fs::read_to_string(pid_path).await?;
		let pid: i32 = pid_str.trim().parse()?;
		assert!(pid > 0);

		if is_pid_running(pid).await? {
			// Process is currently running
			return Ok(ProcessState::Running { pid });
		} else {
			// Process did not successfully write exit code to file system.
			//
			// This happens when the process manager does not exit gracefully. For example, on a
			// system restart or force kill process.
			//
			// There is a rare race condition when:
			// - process_state started
			// - attempts to read exit code
			// - process crashes
			// - arrives here and pid no longer is running
			return Ok(ProcessState::Exited { exit_code: None });
		}
	}

	// If process does not have a PID yet, it's starting
	Ok(ProcessState::Starting)
}

/// Kills a process.
async fn kill_process(
	process_id: Uuid,
	kill_grace: Duration,
	base_data_dir: &PathBuf,
) -> Result<bool> {
	// Wait for process to start if race condition with start command
	let pid = loop {
		match get_process_state(process_id, base_data_dir).await? {
			ProcessState::Starting => {}
			ProcessState::Running { pid } => break pid,
			ProcessState::Exited { .. } | ProcessState::NotFound => {
				return Ok(false);
			}
		}

		tokio::time::sleep(Duration::from_millis(100)).await;
	};

	#[cfg(unix)]
	{
		use nix::errno::Errno;
		use nix::sys::signal::{kill, Signal};
		use nix::unistd::Pid;
		use tokio::time::{sleep, Duration, Instant};

		assert!(pid > 0);

		// Send SIGTERM
		//
		// Supervisor will forward the signal to the children
		match tokio::task::block_in_place(|| kill(Pid::from_raw(pid), Signal::SIGTERM)) {
			Result::Ok(_) => {}
			Err(Errno::ESRCH) => return Ok(false),
			Err(e) => return Err(anyhow::anyhow!("Failed to send SIGTERM: {}", e)),
		}

		// Poll for process exit
		let start = Instant::now();
		while start.elapsed() < kill_grace {
			sleep(Duration::from_millis(100)).await;
			if !is_pid_running(pid).await? {
				return Ok(true);
			}
		}

		// Send SIGKILL if process hasn't exited
		match tokio::task::block_in_place(|| kill(Pid::from_raw(pid), Signal::SIGKILL)) {
			Result::Ok(_) => Ok(true),
			Err(Errno::ESRCH) => Ok(true), // Assume process was already killed by SIGTERM in race condition
			Err(e) => Err(anyhow::anyhow!("Failed to send SIGKILL: {}", e)),
		}
	}

	#[cfg(windows)]
	{
		use tokio::time::{sleep, Duration, Instant};
		use windows::Win32::Foundation::CloseHandle;
		use windows::Win32::Foundation::HANDLE;
		use windows::Win32::System::Threading::{OpenProcess, TerminateProcess, PROCESS_TERMINATE};

		unsafe {
			// Open the process
			let process_handle: HANDLE = OpenProcess(PROCESS_TERMINATE, false, pid as u32)?;
			if process_handle.is_invalid() {
				return Ok(false);
			}

			// Attempt to terminate the process gracefully
			if TerminateProcess(process_handle, 0).as_bool() {
				// Poll for process exit
				let start = Instant::now();
				while start.elapsed() < kill_grace {
					sleep(Duration::from_millis(100)).await;
					if !is_pid_running(pid).await? {
						CloseHandle(process_handle);
						return Ok(true);
					}
				}

				// Force terminate if process hasn't exited
				if TerminateProcess(process_handle, 1).as_bool() {
					CloseHandle(process_handle);
					Ok(true)
				} else {
					CloseHandle(process_handle);
					Err(anyhow::anyhow!("Failed to terminate process"))
				}
			} else {
				CloseHandle(process_handle);
				Err(anyhow::anyhow!("Failed to initiate process termination"))
			}
		}
	}
}

/// Checks if a PID is running in a cross-platform way.
///
/// Should only be called by `process_state`.
async fn is_pid_running(pid: i32) -> Result<bool> {
	tokio::task::block_in_place(move || {
		#[cfg(unix)]
		{
			use nix::errno::Errno;
			use nix::sys::signal::kill;
			use nix::unistd::Pid;

			match kill(Pid::from_raw(pid), None) {
				Result::Ok(_) => Ok(true),      // Process exists
				Err(Errno::ESRCH) => Ok(false), // No process
				Err(Errno::EPERM) => bail!("does not have permission to check process status"),
				Err(e) => {
					bail!("unexpected error when checking process existence: {}", e)
				}
			}
		}

		#[cfg(windows)]
		{
			use windows::Win32::Foundation::{CloseHandle, HANDLE};
			use windows::Win32::System::Threading::{
				OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION,
			};

			unsafe {
				let handle: HANDLE =
					match OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid as u32) {
						Result::Ok(handle) => handle,
						Err(_) => return Ok(false), // Process doesn't exist or we don't have permission
					};

				if handle.is_invalid() {
					Ok(false)
				} else {
					CloseHandle(handle);
					Ok(true)
				}
			}
		}
	})
}

/// Wait for a PID to exit.
///
/// Should only be called by `ProcessManager::start`.
async fn wait_pid_exit(pid: i32) -> Result<()> {
	// Wait for the process to exit in a cross-platform way
	#[cfg(unix)]
	{
		use nix::{errno::Errno, sys::signal::kill, unistd::Pid};
		use std::time::Duration;

		tokio::task::spawn_blocking(move || {
			loop {
				match kill(Pid::from_raw(pid), None) {
					Result::Ok(_) => {
						// Process still exists, continue waiting
						std::thread::sleep(Duration::from_millis(100));
					}
					Err(Errno::ESRCH) => {
						// Process no longer exists
						return Ok(());
					}
					Err(Errno::EPERM) => {
						bail!("does not have permission to check process status")
					}
					Err(e) => {
						bail!("Error checking process: {}", e);
					}
				}
			}
		})
		.await??;
	}

	#[cfg(windows)]
	{
		use windows::Win32::{
			Foundation::{CloseHandle, HANDLE},
			System::Threading::{OpenProcess, WaitForSingleObject, INFINITE, PROCESS_SYNCHRONIZE},
		};

		let handle = unsafe {
			match OpenProcess(PROCESS_SYNCHRONIZE, false, pid as u32) {
				Result::Ok(handle) => handle,
				Err(_) => return Err(anyhow!("Failed to open process handle")),
			}
		};

		if handle.is_invalid() {
			return Err(anyhow!("Failed to open process handle"));
		}

		tokio::task::spawn_blocking(move || unsafe {
			WaitForSingleObject(handle, INFINITE);
			CloseHandle(handle);
		})
		.await?;
	}

	Ok(())
}

/// Spawns an orphaned process.
///
/// This allows us to run processes that will stay running even after the parent exits.
fn spawn_orphaned_process(
	process_supervisor_path: PathBuf,
	process_data_dir: PathBuf,
	current_dir: &str,
	program: &str,
	args: &[&str],
	envs: &[(String, String)],
) -> Result<()> {
	// Prepare the arguments for the process supervisor
	let mut supervisor_args = vec![process_data_dir.to_str().unwrap(), current_dir, program];
	supervisor_args.extend(args.iter().map(|&s| s));

	#[cfg(target_family = "unix")]
	{
		Command::new(&process_supervisor_path)
			.args(&supervisor_args)
			.envs(envs.iter().cloned())
			.stdin(Stdio::null())
			.stdout(Stdio::null())
			.stderr(Stdio::null())
			.spawn()?;

		Ok(())

		// TODO: This is the correct way to oprhan a process. Use the process manager instead.
		// use nix::{
		// 	sys::wait::{waitpid, WaitStatus},
		// 	unistd::{fork, ForkResult},
		// };
		// use std::os::unix::process::CommandExt;
		//
		// match unsafe { fork() }.context("process first fork failed")? {
		// 	ForkResult::Parent { child } => {
		// 		// Ensure that the child process spawned successfully
		// 		match waitpid(child, None).context("waitpid failed")? {
		// 			WaitStatus::Exited(_, 0) => Ok(()),
		// 			WaitStatus::Exited(_, status) => {
		// 				bail!("Child process exited with status {}", status)
		// 			}
		// 			_ => bail!("Unexpected wait status for child process"),
		// 		}
		// 	}
		// 	ForkResult::Child => {
		// 		// Child process
		// 		match unsafe { fork() } {
		// 			Result::Ok(ForkResult::Parent { .. }) => {
		// 				// Exit the intermediate child
		// 				std::process::exit(0);
		// 			}
		// 			Result::Ok(ForkResult::Child) => {
		// 				// Exit immediately on fail in order to not leak process
		// 				let err = Command::new(&process_supervisor_path)
		// 					.args(&supervisor_args)
		// 					.envs(envs.iter().cloned())
		// 					.stdin(Stdio::null())
		// 					.stdout(Stdio::null())
		// 					.stderr(Stdio::null())
		// 					.exec();
		// 				eprintln!("exec failed: {err:?}");
		// 				std::process::exit(1);
		// 			}
		// 			Err(err) => {
		// 				// Exit immediately in order to not leak child process.
		// 				//
		// 				// The first fork doesn't need to exit on error since it
		// 				eprintln!("process second fork failed: {err:?}");
		// 				std::process::exit(1);
		// 			}
		// 		}
		// 	}
		// }
	}

	#[cfg(target_os = "windows")]
	{
		use std::os::windows::process::CommandExt;
		use windows::Win32::System::Threading::{CREATE_NEW_PROCESS_GROUP, DETACHED_PROCESS};

		// Windows implementation remains the same
		Command::new(&process_supervisor_path)
			.args(&supervisor_args)
			.envs(envs.iter().cloned())
			.creation_flags(CREATE_NEW_PROCESS_GROUP.0 | DETACHED_PROCESS.0)
			.stdin(Stdio::null())
			.stdout(Stdio::null())
			.stderr(Stdio::null())
			.spawn()?;
		Ok(())
	}
}

/// Reads a log file and streams lines as they're received.
async fn tail_logs(path: PathBuf, task: TaskCtx, stream_name: &'static str) -> Result<()> {
	let file = File::open(&path).await?;
	let mut reader = BufReader::new(file);
	let mut buffer = String::new();

	loop {
		match reader.read_line(&mut buffer).await {
			Result::Ok(0) => {
				// Reached EOF, wait a bit before checking for new content
				tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
				continue;
			}
			Result::Ok(_) => {
				// Trim the newline character
				let line = buffer.trim_end();
				task.log(format!("[{}] {}", stream_name, line));
				buffer.clear();
			}
			Err(e) => return Err(e.into()),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::time::Duration;
	use tokio::{
		sync::{broadcast, mpsc},
		time::sleep,
	};

	use crate::util::task::{TaskCtxInner, TaskEvent};

	async fn setup_test_environment() -> Result<(ProcessManager, tempfile::TempDir)> {
		// Set up a temporary directory for the test
		let temp_dir = tempfile::tempdir()?;
		let temp_path = temp_dir.path().to_path_buf();

		// Create a fake project in the meta file
		let api_endpoint = "https://fake.api.endpoint".to_string();
		let cloud_token = "fake_cloud_token".to_string();
		crate::config::meta::insert_project(&temp_path, api_endpoint, cloud_token).await?;

		// Create a ProcessManager with default kill_grace
		let process_manager = ProcessManager {
			key: "test_process",
			kill_grace: Duration::from_secs(5),
		};

		Ok((process_manager, temp_dir))
	}

	fn create_task_ctx() -> (TaskCtx, mpsc::UnboundedReceiver<TaskEvent>) {
		let (log_tx, log_rx) = mpsc::unbounded_channel();
		let (_shutdown_tx, shutdown_rx) = broadcast::channel(1);
		let task = TaskCtxInner::new(log_tx, shutdown_rx);
		(task, log_rx)
	}

	#[tokio::test(flavor = "multi_thread")]
	async fn test_process_manager_lifecycle() -> Result<()> {
		let (process_manager, temp_dir) = setup_test_environment().await?;

		// Create a TaskCtx
		let (task, mut log_rx) = create_task_ctx();

		// Start a process that logs to stdout and stderr, then exits
		let command = "sh".to_string();
		let args = vec![
			"-c".to_string(),
			r#"
				echo "ENV_VAR: $ENV_VAR"
				echo 'Hello from stdout'
				echo 'Error message' >&2
				sleep 1
				echo 'Exiting now'
				exit 42
			"#
			.to_string(),
		];
		let envs = vec![("ENV_VAR".to_string(), "test_value".to_string())];
		let current_dir = std::env::current_dir()?.to_string_lossy().to_string();
		let base_data_dir = temp_dir.path().to_path_buf();

		// Start the process
		let handle = tokio::spawn({
			let process_manager = process_manager.clone();
			let base_data_dir = base_data_dir.clone();
			async move {
				process_manager
					.start(StartOpts {
						task,
						command,
						args,
						envs,
						current_dir,
						base_data_dir,
					})
					.await
			}
		});

		// Collect logs
		let mut stdout_logs = Vec::new();
		let mut stderr_logs = Vec::new();
		while let Some(event) = log_rx.recv().await {
			match event {
				TaskEvent::Log(log) => {
					if log.contains("[stdout]") {
						stdout_logs.push(log);
					} else if log.contains("[stderr]") {
						stderr_logs.push(log);
					}
				}
				TaskEvent::Result { .. } => break,
			}
		}

		// Wait for the process to finish and get the exit code
		let exit_code = handle.await??;

		// Verify process is not running
		assert!(!process_manager.is_running(&base_data_dir).await?);

		// Verify exit code
		assert_eq!(exit_code, Some(42));

		// Verify logs
		assert!(stdout_logs
			.iter()
			.any(|log| log.contains("ENV_VAR: test_value")));
		assert!(stdout_logs
			.iter()
			.any(|log| log.contains("Hello from stdout")));
		assert!(stdout_logs.iter().any(|log| log.contains("Exiting now")));
		assert!(stderr_logs.iter().any(|log| log.contains("Error message")));

		// Restart the process
		let command = "sh".to_string();
		let args = vec![
			"-c".to_string(),
			"echo 'Restarted process'; sleep 2; exit 0".to_string(),
		];
		let envs = Vec::new();
		let current_dir = std::env::current_dir()?.to_string_lossy().to_string();
		let base_data_dir = temp_dir.path().to_path_buf();

		let (task, _log_rx) = create_task_ctx();
		let handle = tokio::spawn({
			let process_manager = process_manager.clone();
			let base_data_dir = base_data_dir.clone();
			async move {
				process_manager
					.start(StartOpts {
						task,
						command,
						args,
						envs,
						current_dir,
						base_data_dir,
					})
					.await
			}
		});

		// Wait a bit to ensure the process has started
		sleep(Duration::from_millis(200)).await;

		// Verify process is running
		assert!(process_manager.is_running(&base_data_dir).await?);

		// Wait for the process to finish
		let exit_code = handle.await??;

		// Verify exit code of restarted process
		assert_eq!(exit_code, Some(0));

		// Verify process is not running after completion
		assert!(!process_manager.is_running(&base_data_dir).await?);

		Ok(())
	}

	#[tokio::test(flavor = "multi_thread")]
	async fn test_process_manager_stop_graceful() -> Result<()> {
		let (process_manager, temp_dir) = setup_test_environment().await?;

		// Create a TaskCtx
		let (task, _log_rx) = create_task_ctx();

		// Start a long-running process with custom exit code on SIGTERM
		let command = "sh".to_string();
		let args = vec![
			"-c".to_string(),
			r#"
				trap 'echo "Exiting with code 42"; exit 42' TERM
				echo 'Starting long process'
				tail -f /dev/null & wait
			"#
			.to_string(),
		];
		let envs = Vec::new();
		let current_dir = std::env::current_dir()?.to_string_lossy().to_string();
		let base_data_dir = temp_dir.path().to_path_buf();

		// Start the process
		let handle = tokio::spawn({
			let process_manager = process_manager.clone();
			let base_data_dir = base_data_dir.clone();
			async move {
				process_manager
					.start(StartOpts {
						task,
						command,
						args,
						envs,
						current_dir,
						base_data_dir,
					})
					.await
			}
		});

		// Wait a bit to ensure the process has started
		sleep(Duration::from_millis(200)).await;

		// Verify process is running
		assert!(
			process_manager.is_running(&base_data_dir).await?,
			"process not running"
		);
		assert!(!handle.is_finished(), "handle not running");

		// Stop the process
		assert!(
			process_manager.stop(&base_data_dir).await?,
			"did not stop process"
		);
		assert!(
			!process_manager.stop(&base_data_dir).await?,
			"stop should not return true if no process"
		);

		// Verify process is not running
		assert!(
			!process_manager.is_running(&base_data_dir).await?,
			"process is still running"
		);

		// Wait for the process to finish and get the exit code with a 1 second timeout
		let exit_code = tokio::time::timeout(Duration::from_secs(1), handle)
			.await
			.expect("timeout waiting for process to finish")??;

		// Verify custom exit code
		assert_eq!(exit_code, Some(42));

		Ok(())
	}

	#[tokio::test(flavor = "multi_thread")]
	async fn test_process_manager_stop_timeout() -> Result<()> {
		let (process_manager, temp_dir) = setup_test_environment().await?;

		// Create a TaskCtx
		let (task, _log_rx) = create_task_ctx();

		// Start a process that ignores SIGTERM
		let command = "sh".to_string();
		let args = vec![
			"-c".to_string(),
			r#"
            trap 'echo "Caught term, ignoring"' TERM
            echo 'Starting process that ignores SIGTERM'
            while true; do
                sleep 1
            done
        "#
			.to_string(),
		];
		let envs = Vec::new();
		let current_dir = std::env::current_dir()?.to_string_lossy().to_string();
		let base_data_dir = temp_dir.path().to_path_buf();

		// Start the process
		let handle = tokio::spawn({
			let process_manager = process_manager.clone();
			let base_data_dir = base_data_dir.clone();
			async move {
				process_manager
					.start(StartOpts {
						task,
						command,
						args,
						envs,
						current_dir,
						base_data_dir,
					})
					.await
			}
		});

		// Wait a bit to ensure the process has started
		sleep(Duration::from_millis(200)).await;

		// Verify process is running
		assert!(
			process_manager.is_running(&base_data_dir).await?,
			"process not running"
		);

		// Attempt to stop the process in the background
		let stop_handle = tokio::spawn({
			let process_manager = process_manager.clone();
			let base_data_dir = base_data_dir.clone();
			async move { process_manager.stop(&base_data_dir).await }
		});

		// Verify the process is still running before the end of the grace period
		sleep(process_manager.kill_grace - Duration::from_millis(500)).await;
		assert!(
			!process_manager.is_running(&base_data_dir).await?,
			"process stopped too early"
		);

		// Verify process is killed after the grace period
		//
		// We've already slept `grace - 500ms` at this point, so we'll check
		// 500ms after the expected kill
		sleep(Duration::from_secs(1)).await;
		assert!(
			!process_manager.is_running(&base_data_dir).await?,
			"process is still running"
		);

		// Wait for the stop task to complete and check its result
		let stop_result = stop_handle.await?;
		assert!(stop_result?, "did not stop process");

		// Wait for the process to finish and get the exit code
		let exit_code = handle.await??;

		// Verify exit code (should be None due to SIGKILL)
		assert_eq!(exit_code, None, "Unexpected exit code");

		Ok(())
	}
}
