mod backend;
mod game_server;

use global_error::prelude::*;
use rivet_api::apis;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::util::task::TaskCtx;

#[derive(Deserialize)]
pub struct Input {
	pub cwd: String,
	pub environment_id: Uuid,
	pub game_server: bool,
	pub backend: bool,
}

#[derive(Serialize)]
pub struct Output {
	game_server: Option<game_server::DeployOutput>,
}

pub struct Task;

impl super::Task for Task {
	type Input = Input;
	type Output = Output;

	const CONFIG: super::TaskConfig = super::TaskConfig {
		// Required for compression & uploads
		prefer_multithreaded: true,
	};

	fn name() -> &'static str {
		"deploy"
	}

	async fn run(task: TaskCtx, input: Self::Input) -> GlobalResult<Self::Output> {
		// Deploy the backend before the game server in order to ensure that new APIs are exposed
		// before the new game server is deployed.

		let ctx = crate::ctx::load().await?;

		// Get the environment
		let backend_project = crate::backend::get_or_create_project(&ctx).await?;
		let backend_environments =
			apis::ee_cloud_backend_projects_envs_api::ee_cloud_backend_projects_envs_list(
				&ctx.openapi_config_cloud,
				&backend_project.project_id.to_string(),
				None,
			)
			.await?;
		let backend_environment = unwrap!(
			backend_environments
				.environments
				.into_iter()
				.find(|x| x.environment_id == input.environment_id),
			"backend not found"
		);

		if input.backend {
			// Backend
			backend::deploy(
				&ctx,
				task.clone(),
				backend::DeployOpts {
					game_id: ctx.game_id.clone(),
					environment_id: input.environment_id.clone(),
					project_path: input.cwd.clone(),
					// TODO:
					skip_migrate: true,
				},
			)
			.await?;
		}

		let game_server = if input.game_server {
			// Game server
			// TODO: Add reading from rivet.json or some sort of build config to read this data. This should
			// support multiple dockerfiles and passing from args/env.
			let deploy = game_server::deploy(
				&ctx,
				task.clone(),
				game_server::DeployOpts {
					backend_environment,
					build_dir: input.cwd.clone(),
				},
			)
			.await?;

			Some(deploy)
		} else {
			None
		};

		// Finish
		task.log_stdout(format!("[Deploy Finished]"));

		Ok(Output { game_server })
	}
}
