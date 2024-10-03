use anyhow::*;
use tokio::process::Command;
use std::collections::HashMap;

use crate::util::task;

/// Runs a command in a cross-platform compatible way.
pub async fn run(task: task::TaskCtx, command: &str, envs: Vec<(String, String)>) -> Result<()> {
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
) -> Result<()> {
	match task.spawn_cmd(command).await {
		Result::Ok(status) => {
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
		Result::Ok(output) => Ok(output),
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

pub fn shell_cmd(cmd: &str) -> Command {
	tokio::process::Command::from(shell_cmd_std(cmd))
}

pub fn shell_cmd_std(cmd: &str) -> std::process::Command {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        use windows::Win32::System::Threading::CREATE_NO_WINDOW;
        let mut cmd = std::process::Command::new(cmd);
        cmd.creation_flags(CREATE_NO_WINDOW.0);
        cmd
    }

    #[cfg(not(windows))]
    {
        // Load the env from the user's profile
        let shell = std::env::var("SHELL").unwrap_or_else(|_| String::from("/bin/sh"));
        let env_command = format!(
            "{} -l -i -c 'env; echo ---ENVIRONMENT_END---'",
            shell
        );
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(&env_command)
            .output()
            .expect("Failed to execute profile loading command");
        let env_output = String::from_utf8(output.stdout)
            .expect("Failed to parse environment output");

        // Parse env vars
        let env_vars: HashMap<String, String> = env_output
            .lines()
            .take_while(|&line| line != "---ENVIRONMENT_END---")
            .filter_map(|line| {
                let mut parts = line.splitn(2, '=');
                Some((
                    parts.next()?.to_string(),
                    parts.next()?.to_string(),
                ))
            })
            .collect();

        // Execute the actual command with the envs from the profile
        let mut cmd = std::process::Command::new(cmd);
        cmd.envs(&env_vars);
        cmd
    }
}
