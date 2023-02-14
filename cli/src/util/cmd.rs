use anyhow::{bail, ensure, Result};

pub async fn execute_docker_cmd(
	mut command: tokio::process::Command,
	error_message: impl std::fmt::Display,
) -> Result<()> {
	match command.output().await {
		Ok(output) => {
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
		Err(err) => {
			if let std::io::ErrorKind::NotFound = err.kind() {
				bail!("Docker not installed, install at https://docs.docker.com/get-docker/")
			} else {
				Err(err.into())
			}
		}
	}
}
