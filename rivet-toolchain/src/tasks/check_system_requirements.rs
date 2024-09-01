use global_error::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::util::{cmd::shell_cmd, task};

#[derive(Deserialize)]
pub struct Input {}

#[derive(Serialize)]
pub struct Output {
	errors: Vec<RequirementError>,
}

#[derive(Serialize)]
pub struct RequirementError {
	title: String,
	body: String,
	docs_url: Option<String>,
}

pub struct Task;

impl task::Task for Task {
	type Input = Input;
	type Output = Output;

	fn name() -> &'static str {
		"check_system_requirements"
	}

	async fn run(_task: task::TaskCtx, _input: Self::Input) -> GlobalResult<Self::Output> {
		let mut errors = Vec::new();

		// Docker
		match tokio::time::timeout(
			Duration::from_secs(5),
			shell_cmd("docker").arg("info").kill_on_drop(true).output(),
		)
		.await
		{
			Ok(Ok(output)) => {
				if !output.status.success() {
					if output.status.code() == Some(127) {
						errors.push(errors::docker_not_found());
					} else {
						errors.push(RequirementError {
							title: "Docker Failed".into(),
							body: format!(
								"Exit code: {}\n\n{}",
								output
									.status
									.code()
									.map_or_else(|| "?".to_string(), |x| x.to_string()),
								String::from_utf8_lossy(&output.stderr).to_string()
							),
							docs_url: None,
						});
					}
				}
			}
			Ok(Err(err)) if err.kind() == std::io::ErrorKind::NotFound => {
				errors.push(errors::docker_not_found())
			}
			Ok(Err(err)) => {
				errors.push(RequirementError {
					title: "Docker Command Error".into(),
					body: err.to_string(),
					docs_url: Some("https://docs.docker.com/get-docker/".into()),
				});
			}
			Err(_) => {
				errors.push(RequirementError {
					title: "Docker Command Timed Out".into(),
					body: "Docker may be paused. Try restarting Docker.".into(),
					docs_url: None,
				});
			}
		}

		Ok(Output { errors })
	}
}

mod errors {
	use super::RequirementError;

	pub fn docker_not_found() -> RequirementError {
		RequirementError {
			title: "Install Docker".into(),
			body: "Docker is required to build & run the game server & backend.".into(),
			docs_url: Some("https://docs.docker.com/get-docker/".into()),
		}
	}
}
