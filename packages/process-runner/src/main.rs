use lazy_static::lazy_static;
use rivet_process_runner_shared as shared;
use std::{
	fs::File,
	io::Write,
	path::Path,
	process::{Child, Command, Stdio},
	sync::{
		atomic::{AtomicBool, Ordering},
		Arc,
	},
	time::Duration,
};
use thiserror::Error;

#[derive(Error, Debug)]
enum ManagerError {
	#[error("Data directory does not exist: {0}")]
	DataDirNotFound(String),
	#[error("Failed to create file: {0}")]
	FileCreationError(#[from] std::io::Error),
	#[error("Failed to write to file: {0}")]
	FileWriteError(std::io::Error),
	#[error("Failed to execute command: {0}")]
	CommandExecutionError(std::io::Error),
	#[error("Failed to send signal: {0}")]
	SignalError(String),
}

lazy_static! {
	static ref HAS_RECEIVED_SIGTERM: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
}

fn main() {
	// Parse arguments
	let args: Vec<String> = std::env::args().collect();
	if args.len() < 4 {
		eprintln!("Error: Invalid number of arguments");
		std::process::exit(1);
	}

	let data_dir = &args[1];
	let current_dir = &args[2];
	let command = &args[3];
	let command_args = &args[4..];

	match run_process(data_dir, current_dir, command, command_args) {
		Ok(exit_code) => std::process::exit(exit_code),
		Err(e) => {
			// Write error to a file
			//
			// We can't recover if this write doesn't work
			let error_path = Path::new(data_dir).join(shared::paths::RUNNER_ERROR);
			let _ = write_to_file(&error_path, &e.to_string());

			std::process::exit(1);
		}
	}
}

fn run_process(
	data_dir: &str,
	current_dir: &str,
	command: &str,
	command_args: &[String],
) -> Result<i32, ManagerError> {
	// Listen for SIGTERM (Unix) and SIGBREAK (Windows)
	ctrlc::set_handler(move || {
		HAS_RECEIVED_SIGTERM.store(true, Ordering::Relaxed);
	}).unwrap();

	// Assert that the data directory exists
	if !Path::new(data_dir).is_dir() {
		return Err(ManagerError::DataDirNotFound(data_dir.to_string()));
	}

	// Write current PID to file
	let pid = std::process::id();
	let pid_path = Path::new(data_dir).join(shared::paths::RUNNER_PID);
	write_to_file(&pid_path, &pid.to_string())?;

	// Open stdout and stderr files for writing
	let stdout_path = Path::new(data_dir).join(shared::paths::CHILD_STDOUT);
	let stderr_path = Path::new(data_dir).join(shared::paths::CHILD_STDERR);
	let stdout = File::create(&stdout_path).map_err(ManagerError::FileCreationError)?;
	let stderr = File::create(&stderr_path).map_err(ManagerError::FileCreationError)?;

	// Run the command
	let mut cmd = Command::new(command);
	cmd.args(command_args)
		.current_dir(current_dir)
		.stdout(Stdio::from(stdout))
		.stderr(Stdio::from(stderr));

	#[cfg(target_os = "windows")]
	{
		use std::os::windows::process::CommandExt;

		// CREATE_NEW_PROCESS_GROUP detaches from this process. This only
		// accepts CTRL_BREAK.
		use windows::Win32::System::Threading::CREATE_NEW_PROCESS_GROUP;
		cmd.creation_flags(CREATE_NEW_PROCESS_GROUP.0);
	}

	let mut child = cmd.spawn().map_err(ManagerError::CommandExecutionError)?;

	// Write child PID to file
	let child_pid = child.id();
	let child_pid_path = Path::new(data_dir).join(shared::paths::CHILD_PID);
	write_to_file(&child_pid_path, &child_pid.to_string())?;

	// Wait for either the child to exit or a signal to be received
	let exit_code = loop {
		if HAS_RECEIVED_SIGTERM.load(Ordering::Relaxed) {
			write_to_file(
				&Path::new(data_dir).join(shared::paths::CHILD_TERMINATING),
				"",
			)?;
			terminate_child(&mut child)?;

			match child.wait() {
				Ok(status) => break status.code(),
				Err(e) => return Err(ManagerError::CommandExecutionError(e)),
			}
		} else {
			match child.try_wait() {
				Ok(Some(status)) => break status.code(),
				Ok(None) => {}
				Err(e) => return Err(ManagerError::CommandExecutionError(e)),
			}
		}

		std::thread::sleep(Duration::from_millis(100));
	};

	// Write exit code to file
	let exit_code_path = Path::new(data_dir).join(shared::paths::CHILD_EXIT_CODE);
	write_to_file(&exit_code_path, &exit_code.map_or_else(||"unknown".to_string(), |x| x.to_string()))?;

	Ok(exit_code.unwrap_or(1))
}

/// Write & flush a string to a file.
fn write_to_file(path: &Path, content: &str) -> Result<(), ManagerError> {
	let mut file = File::create(path).map_err(ManagerError::FileCreationError)?;
	writeln!(file, "{}", content).map_err(ManagerError::FileWriteError)?;
	file.flush().map_err(ManagerError::FileWriteError)?;
	Ok(())
}

#[cfg(unix)]
fn terminate_child(child: &mut Child) -> Result<(), ManagerError> {
	use nix::{
		sys::signal::{kill, Signal},
		unistd::Pid,
	};

	let pid = Pid::from_raw(child.id() as i32);
	kill(pid, Signal::SIGTERM).map_err(|e| ManagerError::SignalError(e.to_string()))
}

#[cfg(windows)]
fn terminate_child(child: &mut Child) -> Result<(), ManagerError> {
	use windows::Win32::System::Console::{GenerateConsoleCtrlEvent, CTRL_BREAK_EVENT};

	unsafe {
		if !GenerateConsoleCtrlEvent(CTRL_BREAK_EVENT, child.id() as u32).as_bool() {
			return Err(ManagerError::SignalError(
				"Failed to generate console control event".to_string(),
			));
		}
	}
	Ok(())
}
