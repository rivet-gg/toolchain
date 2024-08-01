mod backend;
mod game_server;

use global_error::prelude::*;
use rivet_api::apis;
use serde::{Deserialize, Serialize};

use crate::util::task::TaskCtx;

#[derive(Deserialize)]
pub struct Input {
	cwd: String,
	environment_id: String,
	game_server: bool,
	backend: bool,
}

#[derive(Serialize)]
pub struct Output {
	version: String,
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

		// TODO: this will return an empty string if not deploying a game server, need to handle
		// this more gracefully
		let mut version = String::new();

		// Reserve image name
		let reserve_res =
			apis::cloud_games_versions_api::cloud_games_versions_reserve_version_name(
				&ctx.openapi_config_cloud,
				&ctx.game_id,
			)
			.await?;
		let display_name = reserve_res.version_display_name;
		task.log_stdout(format!("[Starting Deploy] {display_name}"));

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

		if input.game_server {
			// Game server
			// TODO: Add reading from rivet.json or some sort of build config to read this data. This should
			// support multiple dockerfiles and passing from args/env.
			let deploy = game_server::deploy(
				&ctx,
				task.clone(),
				game_server::DeployOpts {
					display_name: display_name.clone(),
					build_dir: input.cwd.clone(),
				},
			)
			.await?;

			// TODO: Move this to the version name tag when ready
			version = deploy.image_id.to_string();
		}

		// Finish
		task.log_stdout(format!("[Deploy Finished] {display_name}"));

		Ok(Output { version })
	}
}
