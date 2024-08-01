use global_error::prelude::*;
use rivet_api::{apis, models};
use serde::{Deserialize, Serialize};

use crate::{backend, util::task::TaskCtx};

#[derive(Deserialize)]
pub struct Input {}

#[derive(Serialize)]
pub struct Output {
	pub token: String,
	pub api_endpoint: String,
	pub game_id: String,
	pub backend_project: models::EeBackendProject,
	pub backend_environments: Vec<models::EeBackendEnvironment>,
}

pub struct Task;

impl super::Task for Task {
	type Input = Input;
	type Output = Output;

	fn name() -> &'static str {
		"get_bootstrap_data"
	}

	async fn run(_task: TaskCtx, _input: Self::Input) -> GlobalResult<Self::Output> {
		let ctx = crate::ctx::load().await?;

		// Get or create backend project
		let backend_project = backend::get_or_create_project(&ctx).await?;
		let backend_environments =
			apis::ee_cloud_backend_projects_envs_api::ee_cloud_backend_projects_envs_list(
				&ctx.openapi_config_cloud,
				&backend_project.project_id.to_string(),
				None,
			)
			.await?
			.environments;

		Ok(Output {
			token: ctx.access_token.clone(),
			api_endpoint: ctx.api_endpoint.clone(),
			game_id: ctx.game_id.clone(),
			backend_project: *backend_project,
			backend_environments,
		})
	}
}
