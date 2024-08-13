mod backend;
mod game_server;

use global_error::prelude::*;
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

		let env = crate::game::get_env(&ctx, input.environment_id).await?;

		if input.backend {
			backend::deploy(
				&ctx,
				task.clone(),
				backend::DeployOpts {
					env: env.clone(),
					project_path: input.cwd.clone(),
					// TODO:
					skip_migrate: true,
				},
			)
			.await?;
		}

		let game_server = if input.game_server {
			// TODO: Add support for configuring in project config.
			// Should support multiple dockerfiles and passing from args/env.

			// Game server
			let deploy = game_server::deploy(
				&ctx,
				task.clone(),
				game_server::DeployOpts {
					env: env.clone(),
					build_dir: input.cwd.clone(),
				},
			)
			.await?;

			Some(deploy)
		} else {
			None
		};

		task.log_stdout(format!("[Deploy Finished]"));

		Ok(Output { game_server })
	}
}
