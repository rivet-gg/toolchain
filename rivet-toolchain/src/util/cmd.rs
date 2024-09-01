use global_error::prelude::*;
use tokio::process::Command;

use crate::util::task;

/// Runs a command in a cross-platform compatible way.
pub async fn run(
	task: task::TaskCtx,
	command: &str,
	envs: Vec<(String, String)>,
) -> GlobalResult<()> {
	if cfg!(unix) {
		let mut cmd = Command::new("/bin/sh");
		cmd.envs(envs).arg("-c").arg(command);
		let build_status = task.spawn_cmd(cmd).await?;
		ensure!(build_status.success(), "command failed");
	} else if cfg!(windows) {
		let mut cmd = Command::new("cmd.exe");
		cmd.envs(envs).arg("/C").arg(command);
		let build_status = task.spawn_cmd(cmd).await?;
		ensure!(build_status.success(), "command failed");
	} else {
		bail!("unknown machine type, expected unix or windows")
	}

	Ok(())
}

/// Run a Docker command with full output.
pub async fn execute_docker_cmd(
	task: task::TaskCtx,
	command: tokio::process::Command,
	error_message: impl std::fmt::Display,
) -> GlobalResult<()> {
	match task.spawn_cmd(command).await {
		Ok(status) => {
			if !status.success() {
				bail!(
					"{error_message} ({})\n\nValidate that Docker is installed and running.",
					status
				);
			}
			Ok(())
		}
		Err(err) => {
			// TODO: This will not correctly handle this error anymore
			// if let std::io::ErrorKind::NotFound = err.kind() {
			// 	bail!("Docker not installed, install at https://docs.docker.com/get-docker/")
			// } else {
			// 	Err(err.into())
			// }
			Err(err)
		}
	}
}

/// Run a Docker command without output.
pub async fn execute_docker_cmd_silent(
	command: tokio::process::Command,
	error_message: impl std::fmt::Display,
) -> GlobalResult<std::process::Output> {
	let output = execute_docker_cmd_silent_fallible(command).await?;
	error_for_output_failure(&output, error_message)?;
	Ok(output)
}

/// Run a Docker command without output and ignore failures.
pub async fn execute_docker_cmd_silent_fallible(
	mut command: tokio::process::Command,
) -> GlobalResult<std::process::Output> {
	match command.output().await {
		Ok(output) => Ok(output),
		Err(err) => {
			if let std::io::ErrorKind::NotFound = err.kind() {
				bail!("Docker not installed, install at https://docs.docker.com/get-docker/")
			} else {
				Err(err.into())
			}
		}
	}
}

/// Throw an error if the output of a command failed.
pub fn error_for_output_failure(
	output: &std::process::Output,
	error_message: impl std::fmt::Display,
) -> GlobalResult<()> {
	if !output.status.success() {
		bail!(
			"{error_message} ({})\n\nstdout:\n{}\n\nstderr:\n{}",
			output.status,
			String::from_utf8_lossy(&output.stdout),
			String::from_utf8_lossy(&output.stderr)
		);
	}

	Ok(())
}

pub fn shell_cmd(cmd: &str) -> Command {
	tokio::process::Command::from(shell_cmd_std(cmd))
}

pub fn shell_cmd_std(cmd: &str) -> std::process::Command {
	if cfg!(windows) {
		// Use native command on Windows
		std::process::Command::new(cmd)
	} else {
		// Load the user's profile & shell on Linux in order to ensure we have the correct $PATH

		let shell = std::env::var("SHELL").unwrap_or_else(|_| String::from("/bin/sh"));

		let mut shell_cmd = std::process::Command::new(&shell);
		shell_cmd
			// Load profile
			.arg("-l")
			// Load rc file
			.arg("-i")
			.arg("-c")
			// Will accept the cmd & all following args
			.arg("\"$@\"")
			// This arg is ignored
			.arg("noop")
			// Pass the actual command
			.arg(cmd);
		shell_cmd
	}
}
