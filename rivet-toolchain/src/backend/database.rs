use global_error::prelude::*;
use rivet_api::apis;
use uuid::Uuid;

use crate::{config, ctx::Ctx, util::task::TaskCtx};

pub async fn provision_database(
	task: TaskCtx,
	ctx: &Ctx,
	project_id: Uuid,
	env_id: Uuid,
) -> GlobalResult<()> {
	task.log_stdout("[Provisioning Database]");

	apis::ee_cloud_backend_projects_envs_api::ee_cloud_backend_projects_envs_provision_database(
		&ctx.openapi_config_cloud,
		&project_id.to_string(),
		&env_id.to_string(),
	)
	.await?;

	// Fetch remote DB URL
	let mut global_project_config = config::global::mutate_project(|config| {
		config
			.opengb
			.projects
			.entry(project_id)
			.or_default()
			.clone()
	})
	.await?;
	let env_config = global_project_config
		.environments
		.entry(env_id)
		.or_default();

	if env_config.url.is_none() {
		task.log_stdout("[Fetching Connection]");

		let db_url_res =
			apis::ee_cloud_backend_projects_envs_api::ee_cloud_backend_projects_envs_get_db_url(
				&ctx.openapi_config_cloud,
				&project_id.to_string(),
				&env_id.to_string(),
			)
			.await?;

		// Add missing db url
		env_config.url = db_url_res.url;

		// Update cache
		config::global::try_mutate_project(|config| {
			// Was inserted in last `mutate_project` call
			let project = unwrap!(config.opengb.projects.get_mut(&project_id));

			project.environments.insert(env_id, env_config.clone());

			Ok(())
		})
		.await?;
	}

	Ok(())
}
