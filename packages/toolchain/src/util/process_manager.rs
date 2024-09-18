use anyhow::*;
use rivet_process_runner_shared as shared;
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
					let process_runner_path =
						rivet_process_runner_embed::get_executable(&base_data_dir)?;
					spawn_orphaned_process(
						process_runner_path,
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
					// Process is still starting. The process runner will write the PID to the
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

					// Get new process state
					match get_process_state(process_id, &base_data_dir).await? {
						ProcessState::Exited { exit_code, error } => {
							// Exit
							if let Some(error) = error {
								bail!("process failed to run: {error}");
							} else {
								return Ok(exit_code);
							}
						}
						x => bail!("process state should be exited, got: {x:?}"),
					}
				}
				ProcessState::Exited { exit_code, error } => {
					// Exited immediately
					if let Some(error) = error {
						bail!("process failed to run: {error}");
					} else {
						return Ok(exit_code);
					}
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
	Exited {
		exit_code: Option<i32>,
		error: Option<String>,
	},

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
	let error_path = process_data_dir.join(shared::paths::RUNNER_ERROR);
	let pid_path = process_data_dir.join(shared::paths::RUNNER_PID);
	if exit_code_path.exists() {
		let exit_code_str = tokio::fs::read_to_string(exit_code_path).await?;
		let exit_code: i32 = exit_code_str.trim().parse()?;
		Ok(ProcessState::Exited {
			exit_code: Some(exit_code),
			error: None,
		})
	} else if error_path.exists() {
		// Read the runner error
		let error = tokio::fs::read_to_string(error_path).await?;
		Ok(ProcessState::Exited {
			exit_code: None,
			error: Some(error),
		})
	} else if pid_path.exists() {
		// Read the PID from the file
		let pid_str = tokio::fs::read_to_string(pid_path).await?;
		let pid: i32 = pid_str.trim().parse()?;
		assert!(pid > 0);

		if is_pid_running(pid).await? {
			// Process is currently running
			Ok(ProcessState::Running { pid })
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
			Ok(ProcessState::Exited {
				exit_code: None,
				error: None,
			})
		}
	} else {
		// If process does not have a PID yet, it's starting
		Ok(ProcessState::Starting)
	}
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
		// Runner will forward the signal to the children
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
			Foundation::CloseHandle,
			Foundation::WAIT_OBJECT_0,
			System::Threading::{OpenProcess, WaitForSingleObject, INFINITE, PROCESS_SYNCHRONIZE},
		};

		let handle = unsafe {
			match OpenProcess(PROCESS_SYNCHRONIZE, false, pid as u32) {
				Result::Ok(handle) => handle,
				Err(_) => return Err(anyhow!("Failed to open process handle")),
			}
		};
		ensure!(!handle.is_invalid(), "failed to open process handle");

		tokio::task::spawn_blocking(move || unsafe {
			match WaitForSingleObject(handle, INFINITE) {
				WAIT_OBJECT_0 => {}
				e => bail!("error waiting for process: {e:?}"),
			};
			CloseHandle(handle);
			Result::Ok(())
		})
		.await??;
	}

	Ok(())
}

/// Spawns an orphaned process.
///
/// This allows us to run processes that will stay running even after the parent exits.
fn spawn_orphaned_process(
	process_runner_path: PathBuf,
	process_data_dir: PathBuf,
	current_dir: &str,
	program: &str,
	args: &[&str],
	envs: &[(String, String)],
) -> Result<()> {
	// Prepare the arguments for the process runner
	let mut runner_args = vec![process_data_dir.to_str().unwrap(), current_dir, program];
	runner_args.extend(args.iter().map(|&s| s));

	#[cfg(target_family = "unix")]
	{
		// Spawn child
		//
		// Calling `.wait()` is required in order to remove zombie processes after complete
		let mut child = Command::new(&process_runner_path)
			.args(&runner_args)
			.envs(envs.iter().cloned())
			.stdin(Stdio::null())
			.stdout(Stdio::null())
			.stderr(Stdio::null())
			.spawn()?;
		tokio::task::spawn_blocking(move || child.wait().expect("child.wait"));

		Ok(())

		// // TODO: This works in unit tests, but doesn't play nice with forking an entire engine.
		// We need to make this fork process runnerunner.
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
		// 				let err = Command::new(&process_runner_path)
		// 					.args(&runner_args)
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
		use windows::Win32::System::Threading::{
			CREATE_NEW_PROCESS_GROUP, CREATE_NO_WINDOW, DETACHED_PROCESS,
		};

		// Spawn process
		//
		// Calling `.wait()` is required in order to remove zombie processes after complete
		let mut child = Command::new(&process_runner_path)
			.args(&runner_args)
			.envs(envs.iter().cloned())
			// .creation_flags(CREATE_NEW_PROCESS_GROUP.0 | DETACHED_PROCESS.0 | CREATE_NO_WINDOW.0)
			// .creation_flags(CREATE_NO_WINDOW.0)
			.stdin(Stdio::null())
			.stdout(Stdio::null())
			.stderr(Stdio::null())
			.spawn()?;
		tokio::task::spawn_blocking(move || child.wait().expect("child.wait"));

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
