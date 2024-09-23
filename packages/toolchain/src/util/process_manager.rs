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

const LOG_POLL_INTERVAL: Duration = Duration::from_millis(100);

#[cfg(unix)]
type PidRaw = i32;

#[cfg(not(unix))]
type PidRaw = u32;

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

					// Spawn process
					let process_runner_path =
						rivet_process_runner_embed::get_executable(&base_data_dir)?;
					spawn_process(
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
							res.context("wait pid exit")?;
						}
						res = tail_logs(stdout_path, task.clone(), "stdout") => {
							res.context("tail logs stdout")?;
							bail!("stdout logs exited early");
						}
						res = tail_logs(stderr_path, task.clone(), "stderr") => {
							res.context("tail logs stderr")?;
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
						x => {
							bail!("process state should be exited, got: {x:?}")
						}
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
	Running { pid: PidRaw },

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

	if error_path.exists() {
		// Read the runner error
		let error = tokio::fs::read_to_string(error_path).await?;
		if error.is_empty() {
			return Ok(ProcessState::Exited {
				exit_code: None,
				error: Some(error),
			});
		}
	}

	if exit_code_path.exists() {
		let exit_code_str = tokio::fs::read_to_string(exit_code_path).await?;
		if !exit_code_str.is_empty() {
			if exit_code_str.trim() == "unknown" {
				return Ok(ProcessState::Exited {
					exit_code: None,
					error: None,
				});
			} else {
				let exit_code: i32 = exit_code_str.trim().parse()?;
				return Ok(ProcessState::Exited {
					exit_code: Some(exit_code),
					error: None,
				});
			}
		}
	}

	if pid_path.exists() {
		// Read the PID from the file
		let pid_str = tokio::fs::read_to_string(pid_path).await?;
		if !pid_str.is_empty() {
			let pid: PidRaw = pid_str.trim().parse()?;
			assert!(pid > 0);

			if is_pid_running(pid)? {
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
				return Ok(ProcessState::Exited {
					exit_code: None,
					error: None,
				});
			}
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
		use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
		use nix::unistd::Pid;
		use std::time::Instant;

		assert!(pid > 0);

		// Send SIGTERM
		//
		// Runner will forward the signal to the children
		match tokio::task::block_in_place(|| kill(Pid::from_raw(pid), Signal::SIGTERM)) {
			Result::Ok(_) => {}
			Err(Errno::ESRCH) => return Ok(false),
			Err(e) => bail!("Failed to send SIGTERM: {}", e),
		}

		// Wait for process to exit
		let start = Instant::now();
		loop {
			match tokio::task::block_in_place(|| {
				waitpid(Pid::from_raw(pid), Some(WaitPidFlag::WNOHANG))
			}) {
				Result::Ok(WaitStatus::Exited(_, _))
				| Result::Ok(WaitStatus::Signaled(_, _, _)) => return Ok(true),
				Result::Ok(_) => {
					if start.elapsed() >= kill_grace {
						break;
					}
					tokio::time::sleep(Duration::from_millis(100)).await;
				}
				Err(Errno::ECHILD) => return Ok(true), // Process has already exited
				Err(e) => bail!("Failed to wait for process: {}", e),
			}
		}

		// Send SIGKILL if process hasn't exited within kill_grace
		terminate_process_tree(Pid::from_raw(pid));

		Ok(true)
	}

	#[cfg(windows)]
	{
		use windows::Win32::{
			Foundation::{CloseHandle, HANDLE, WAIT_OBJECT_0, WAIT_TIMEOUT},
			System::{
				Console::{GenerateConsoleCtrlEvent, CTRL_BREAK_EVENT},
				Threading::{OpenProcess, WaitForSingleObject, PROCESS_SYNCHRONIZE},
			},
		};

		unsafe {
			// Attempt to terminate the process gracefully
			if GenerateConsoleCtrlEvent(CTRL_BREAK_EVENT, pid as u32).as_bool() {
				// Open the process
				let process_handle: HANDLE = OpenProcess(PROCESS_SYNCHRONIZE, false, pid as u32)?;
				if process_handle.is_invalid() {
					return Ok(true);
				}

				// Wait for process exit
				match tokio::task::block_in_place(|| {
					WaitForSingleObject(process_handle, kill_grace.as_millis() as u32)
				}) {
					WAIT_OBJECT_0 => {
						CloseHandle(process_handle);
						Ok(true)
					}
					WAIT_TIMEOUT => {
						CloseHandle(process_handle);

						// Process didn't exit within grace period, force terminate process & all children
						terminate_process_tree(pid);

						// HACK: Sleep to allow process to finish terminating
						tokio::time::sleep(Duration::from_secs(1)).await;

						Ok(true)
					}
					err => {
						CloseHandle(process_handle);
						bail!("WaitForSingleObject failed: {err:?}")
					}
				}
			} else {
				bail!("failed to terminate process")
			}
		}
	}
}

#[cfg(unix)]
fn terminate_process_tree(pid: nix::unistd::Pid) {
	use nix::sys::signal::{kill, Signal};
	use nix::unistd::Pid;
	use std::fs;
	use std::path::Path;

	// TODO: This does not work on macOS
	// List all child processes
	if let Result::Ok(entries) = fs::read_dir(Path::new("/proc")) {
		for entry in entries.flatten() {
			if let Result::Ok(file_name) = entry.file_name().into_string() {
				if let Result::Ok(child_pid) = file_name.parse::<i32>() {
					if let Result::Ok(content) =
						fs::read_to_string(format!("/proc/{}/stat", child_pid))
					{
						let parts: Vec<&str> = content.split_whitespace().collect();
						if parts.len() > 3 && parts[3] == pid.to_string() {
							// This is a child process, kill it
							let _ = terminate_process_tree(Pid::from_raw(child_pid));
						}
					}
				}
			}
		}
	}

	// Then, kill the process itself
	let _ = kill(pid, Signal::SIGKILL);
}

#[cfg(windows)]
fn terminate_process_tree(pid: PidRaw) {
	use windows::Win32::System::{
		Diagnostics::ToolHelp::{
			CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32,
			TH32CS_SNAPPROCESS,
		},
		Threading::{OpenProcess, TerminateProcess, PROCESS_TERMINATE},
	};

	unsafe {
		// Gather child PIDs
		let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).unwrap();
		let mut entry = PROCESSENTRY32::default();
		entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;
		let mut child_pids = Vec::new();
		if Process32First(snapshot, &mut entry).as_bool() {
			loop {
				if entry.th32ParentProcessID == pid {
					// eprintln!("Child exec file: {}", String::from_utf8_lossy(&entry.szExeFile));
					child_pids.push(entry.th32ProcessID)
				}

				if !Process32Next(snapshot, &mut entry).as_bool() {
					break;
				}
			}
		}

		// Kill this process before killing children in order to prevent the
		// parent from doing anything more to the child processes
		match OpenProcess(PROCESS_TERMINATE, false, pid) {
			Result::Ok(handle) => {
				if !handle.is_invalid() {
					// Kill process
					if !TerminateProcess(handle, 1).as_bool() {
						eprintln!("failed to kill process");
					} else {
						eprintln!("terminated process");
					}
				} else {
					eprintln!("handle invalid: {pid}");
				}
			}
			Err(_) => {
				eprintln!("failed to open process, likely already stopped");
			}
		}

		loop {
			if is_pid_running(pid).unwrap() {
				eprintln!("pid still running: {pid}");
				std::thread::sleep(Duration::from_millis(500));
			} else {
				break;
			}
		}

		// Recursively kill child processes immediately
		for pid in child_pids {
			terminate_process_tree(pid);
		}
	}
}

/// Checks if a PID is running in a cross-platform way.
///
/// Should only be called by `process_state`.
fn is_pid_running(pid: PidRaw) -> Result<bool> {
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
			use windows::Win32::Foundation::{CloseHandle, HANDLE, STILL_ACTIVE};
			use windows::Win32::System::Threading::{
				GetExitCodeProcess, OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION,
			};

			unsafe {
				let handle: HANDLE =
					match OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid as u32) {
						Result::Ok(handle) => handle,
						Err(_) => return Ok(false), // Process doesn't exist or we don't have permission
					};

				if handle.is_invalid() {
					return Ok(false);
				}

				let mut exit_code = 0u32;
				let success = GetExitCodeProcess(handle, &mut exit_code as *mut u32);
				CloseHandle(handle);

				return Ok(success.as_bool() && exit_code == STILL_ACTIVE.0 as u32);
			}
		}
	})
}

/// Wait for a PID to exit.
///
/// Should only be called by `ProcessManager::start`.
async fn wait_pid_exit(pid: PidRaw) -> Result<()> {
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
				Err(_) => bail!("Failed to open process handle"),
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

	// HACK: Add grace period to allow logs to finish reading
	tokio::time::sleep(LOG_POLL_INTERVAL + Duration::from_millis(50)).await;

	Ok(())
}

fn spawn_process(
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

	// Spawn child
	//
	// Calling `.wait()` is required in order to remove zombie processes after complete
	let mut cmd = Command::new(&process_runner_path);
	cmd.args(&runner_args)
		.envs(envs.iter().cloned())
		.stdin(Stdio::null())
		.stdout(Stdio::null())
		.stderr(Stdio::null());

	#[cfg(target_os = "windows")]
	{
		use std::os::windows::process::CommandExt;
		use windows::Win32::System::Threading::CREATE_NEW_PROCESS_GROUP;
		cmd.creation_flags(CREATE_NEW_PROCESS_GROUP.0);
	}

	let mut child = cmd.spawn()?;

	tokio::task::spawn_blocking(move || child.wait().expect("child.wait"));

	Ok(())
}

/// Reads a log file and streams lines as they're received.
async fn tail_logs(path: PathBuf, task: TaskCtx, stream_name: &'static str) -> Result<()> {
	let file = File::open(&path).await?;
	let reader = BufReader::new(file);

	// `read_line` is not cancellation safe, we have to sue `lines`
	let mut lines = reader.lines();

	// This will run indefinitely until `wait_pid_exit` finishes
	loop {
		match lines.next_line().await {
			Result::Ok(Some(line)) => {
				// Trim the newline character
				let line = line.trim_end();
				task.log(format!("[{}] {}", stream_name, line));
			}
			Result::Ok(None) => {
				// Reached EOF, wait a bit before checking for new content
				tokio::time::sleep(LOG_POLL_INTERVAL).await;
				continue;
			}
			Err(e) => {
				let buf = lines.get_ref().buffer();
				return Err(e).context(format!(
					"tail log line ({} bytes, lossy: {})",
					buf.len(),
					String::from_utf8_lossy(buf)
				));
			}
		}
	}
}
