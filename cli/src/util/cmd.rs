use anyhow::{bail, Result};

/// Run a Docker command with full output.
pub async fn execute_docker_cmd(
	mut command: tokio::process::Command,
	error_message: impl std::fmt::Display,
) -> Result<()> {
	match command.status().await {
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
			if let std::io::ErrorKind::NotFound = err.kind() {
				bail!("Docker not installed, install at https://docs.docker.com/get-docker/")
			} else {
				Err(err.into())
			}
		}
	}
}

/// Run a Docker command without output.
pub async fn execute_docker_cmd_silent(
	command: tokio::process::Command,
	error_message: impl std::fmt::Display,
) -> Result<std::process::Output> {
	let output = execute_docker_cmd_silent_fallible(command).await?;
	error_for_output_failure(&output, error_message)?;
	Ok(output)
}

/// Run a Docker command without output and ignore failures.
pub async fn execute_docker_cmd_silent_fallible(
	mut command: tokio::process::Command,
) -> Result<std::process::Output> {
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
) -> Result<()> {
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

/// Throw an error if the output of a command failed.
pub async fn read_stdout_fallible(mut command: tokio::process::Command) -> Result<String> {
	let output = command.output().await?;

	if !output.status.success() {
		bail!(
			"Command failed ({})\n\nstdout:\n{}\n\nstderr:\n{}",
			output.status,
			String::from_utf8_lossy(&output.stdout),
			String::from_utf8_lossy(&output.stderr)
		);
	}

	let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();

	Ok(stdout)
}
