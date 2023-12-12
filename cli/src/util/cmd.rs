use global_error::prelude::*;
use tokio::process::Command;

/// Runs a script in a cross-platform compatible way.
pub async fn run_script(script: &str, envs: Vec<(String, String)>) -> GlobalResult<()> {
	if cfg!(unix) {
		let mut cmd = Command::new("/bin/sh");
		remove_rivet_env(&mut cmd);
		cmd.envs(envs).arg("-c").arg(script);
		let build_status = cmd.status().await?;
		ensure!(build_status.success(), "command failed");
	} else if cfg!(windows) {
		let mut cmd = Command::new("cmd.exe");
		remove_rivet_env(&mut cmd);
		cmd.envs(envs).arg("/C").arg(script);
		let build_status = cmd.status().await?;
		ensure!(build_status.success(), "command failed");
	} else {
		bail!("unknown machine type, expected unix or windows")
	}

	Ok(())
}

/// Remove environment variables specific to Rivet in order to ensure the child command uses our
/// sanitized environment variables.
///
/// Specifically: `RIVET_TOKEN` should not be accidentally passed to some commands like the CLI.
fn remove_rivet_env(cmd: &mut Command) {
	for (k, _) in std::env::vars() {
		if k.starts_with("RIVET_") {
			cmd.env_remove(&k);
		}
	}
}

/// Run a Docker command with full output.
pub async fn execute_docker_cmd(
	mut command: tokio::process::Command,
	error_message: impl std::fmt::Display,
) -> GlobalResult<()> {
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
