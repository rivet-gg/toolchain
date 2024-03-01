use crate::commands::token;
use global_error::prelude::*;
use tokio::process::Command;

/// Runs a command in a cross-platform compatible way.
pub async fn run(command: &str, envs: Vec<(String, String)>) -> GlobalResult<()> {
	if cfg!(unix) {
		let mut cmd = Command::new("/bin/sh");
		remove_rivet_env(&mut cmd);
		cmd.envs(envs).arg("-c").arg(command);
		let build_status = cmd.status().await?;
		ensure!(build_status.success(), "command failed");
	} else if cfg!(windows) {
		let mut cmd = Command::new("cmd.exe");
		remove_rivet_env(&mut cmd);
		cmd.envs(envs).arg("/C").arg(command);
		let build_status = cmd.status().await?;
		ensure!(build_status.success(), "command failed");
	} else {
		bail!("unknown machine type, expected unix or windows")
	}

	Ok(())
}

/// Token to issue for command
pub enum RunWithRivetToken {
	ThisMachine,
	RivetServers,
}

pub struct RunWithRivetOpts<'a> {
	pub command: &'a str,
	pub env: Vec<(String, String)>,

	/// Namespace to execute against
	pub namespace: Option<&'a str>,

	/// Token to issue for command
	pub token: RunWithRivetToken,
}

/// Runs a command with `run_command` and populates `RIVET_API_ENDPOINT` and `RIVET_TOKEN` env vars.
pub async fn run_with_rivet(ctx: &cli_core::Ctx, opts: RunWithRivetOpts<'_>) -> GlobalResult<()> {
	// Generate token
	let token = match opts.token {
		RunWithRivetToken::ThisMachine => {
			token::create::dev::execute(
				ctx,
				&token::create::dev::Opts {
					namespace: opts.namespace.map(String::from),
				},
			)
			.await?
			.token
		}
		RunWithRivetToken::RivetServers => {
			token::create::pub_ns::execute(
				ctx,
				&token::create::pub_ns::Opts {
					namespace: opts.namespace.map(String::from),
				},
			)
			.await?
			.token
		}
	};

	// Run command
	let mut envs = vec![
		("RIVET_API_ENDPOINT".into(), ctx.api_endpoint.clone()),
		("RIVET_TOKEN".into(), token),
	];
	if let Some(namespace) = opts.namespace {
		envs.push(("RIVET_NAMESPACE".into(), namespace.into()));
	}
	envs.extend(opts.env);
	run(&opts.command, envs).await?;

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
