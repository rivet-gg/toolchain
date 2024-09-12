use anyhow::Result;
use std::{fs, path::Path};

pub const PROCESS_SUPERVISOR_BINARY: &[u8] = include_bytes!(env!("PROCESS_SUPERVISOR_BINARY_PATH"));
pub const PROCESS_SUPERVISOR_BINARY_HASH: &str = env!("PROCESS_SUPERVISOR_BINARY_HASH");

pub fn get_executable(data_dir: &Path) -> Result<std::path::PathBuf> {
	let binary_name = if cfg!(windows) {
		"process-supervisor.exe"
	} else {
		"process-supervisor"
	};
	let hash_dir = data_dir
		.join("process-supervisor")
		.join(PROCESS_SUPERVISOR_BINARY_HASH);
	let executable_path = hash_dir.join(binary_name);

	if !executable_path.exists() {
		fs::create_dir_all(&hash_dir)?;
		fs::write(&executable_path, PROCESS_SUPERVISOR_BINARY)?;

		#[cfg(unix)]
		{
			use std::os::unix::fs::PermissionsExt;
			let mut perms = fs::metadata(&executable_path)?.permissions();
			perms.set_mode(0o755);
			fs::set_permissions(&executable_path, perms)?;
		}
	}

	Ok(executable_path)
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::process::Command;
	use std::thread::sleep;
	use std::time::Duration;

	#[test]
	fn test_get_executable() {
		let temp_dir = tempfile::tempdir().unwrap();
		let data_dir = temp_dir.path();

		let executable_path = get_executable(data_dir).unwrap();

		assert!(executable_path.exists(), "Executable file should exist");
		assert!(executable_path.is_file(), "Path should point to a file");

		// Check if the file is executable
		#[cfg(unix)]
		{
			use std::os::unix::fs::PermissionsExt;

			let metadata = fs::metadata(&executable_path).unwrap();
			let permissions = metadata.permissions();
			assert_eq!(
				permissions.mode() & 0o111,
				0o111,
				"File should have executable permissions"
			);
		}

		#[cfg(windows)]
		{
			// On Windows, we can't easily check executable permissions,
			// so we'll just ensure the file exists with the correct extension
			assert!(
				executable_path.to_str().unwrap().ends_with(".exe"),
				"Windows executable should have .exe extension"
			);
		}

		// Run test command
		let output = Command::new(&executable_path)
			.args(&[
				data_dir.to_str().unwrap(),
				temp_dir.path().to_str().unwrap(),
				"echo",
				"Hello, World!",
			])
			.output()
			.expect("Failed to execute command");

		// Wait a bit for files to be written
		sleep(Duration::from_millis(100));

		// Read stdout from file
		let stdout_path = data_dir.join("stdout");
		let stdout_content = fs::read_to_string(stdout_path).expect("Failed to read stdout file");
		assert_eq!(
			stdout_content.trim(),
			"Hello, World!",
			"Echo command should output 'Hello, World!' to stdout file"
		);

		// Read PID from file
		let pid_path = data_dir.join("pid");
		let pid_content = fs::read_to_string(pid_path).expect("Failed to read PID file");
		let pid: u32 = pid_content.trim().parse().expect("Failed to parse PID");
		assert!(pid > 0, "PID should be a positive number");

		// Read exit code from file
		let exit_code_path = data_dir.join("exit_code");
		let exit_code_content =
			fs::read_to_string(exit_code_path).expect("Failed to read exit code file");
		let exit_code: i32 = exit_code_content
			.trim()
			.parse()
			.expect("Failed to parse exit code");
		assert_eq!(exit_code, 0, "Exit code in file should be 0");

		// Check exit code from command output
		assert_eq!(
			output.status.code(),
			Some(0),
			"Command should exit with status code 0"
		);

		// Clean up
		fs::remove_dir_all(data_dir).unwrap();
	}
}
