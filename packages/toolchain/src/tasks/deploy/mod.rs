use std::collections::HashMap;

use anyhow::*;
use rivet_api::{apis, models};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
	build,
	project::environment::TEMPEnvironment,
	ToolchainCtx,
	{config, util::task},
};

mod docker;
mod js;

#[derive(Deserialize)]
pub struct Input {
	pub config: config::Config,
	pub environment_id: Uuid,
}

#[derive(Serialize)]
pub struct Output {}

pub struct Task;

impl task::Task for Task {
	type Input = Input;
	type Output = Output;

	fn name() -> &'static str {
		"deploy"
	}

	async fn run(task: task::TaskCtx, input: Self::Input) -> Result<Self::Output> {
		let ctx = crate::toolchain_ctx::load().await?;

		let env = crate::project::environment::get_env(&ctx, input.environment_id).await?;

		// Reserve version name
		let reserve_res =
			apis::cloud_games_versions_api::cloud_games_versions_reserve_version_name(
				&ctx.openapi_config_cloud,
				&ctx.project.game_id.to_string(),
			)
			.await?;
		let version_name = reserve_res.version_display_name;

		for build in &input.config.builds {
			let _build_id = build_and_upload(
				&ctx,
				task.clone(),
				input.config.clone(),
				&env,
				&version_name,
				build,
			)
			.await?;
		}

		task.log("");
		task.log("[Deploy Finished]");

		Ok(Output {})
	}
}

/// Builds the required resources and uploads it to Rivet.
///
/// Returns the resulting build ID.
async fn build_and_upload(
	ctx: &ToolchainCtx,
	task: task::TaskCtx,
	config: config::Config,
	env: &TEMPEnvironment,
	version_name: &str,
	build: &config::Build,
) -> Result<Uuid> {
	// Build tags
	//
	// **version**
	//
	// Unique ident for this build. Used for figuring out which server to start when
	// passing dynamic version from client.
	//
	// **latest**
	//
	// Indicates the latest build to use for this environment. Used if not providing a client-side
	// version.
	let mut tags = HashMap::from([
		(build::tags::VERSION.to_string(), version_name.to_string()),
		(build::tags::CURRENT.to_string(), "true".to_string()),
	]);
	tags.extend(build.tags.clone());

	let exclusive_tags = vec![
		build::tags::VERSION.to_string(),
		build::tags::CURRENT.to_string(),
	];

	// Build & upload
	let build_id = match &build.runtime {
		config::build::Runtime::Docker(docker) => {
			docker::build_and_upload(
				&ctx,
				task.clone(),
				docker::BuildAndUploadOpts {
					env: env.clone(),
					config: config.clone(),
					build_config: docker.clone(),
					version_name: version_name.to_string(),
				},
			)
			.await?
		}
		config::build::Runtime::JavaScript(js) => {
			js::build_and_upload(
				&ctx,
				task.clone(),
				js::BuildAndUploadOpts {
					env: env.clone(),
					config: config.clone(),
					build_config: js.clone(),
					version_name: version_name.to_string(),
				},
			)
			.await?
		}
	};

	// Tag build
	let complete_res = apis::actor_builds_api::actor_builds_patch_tags(
		&ctx.openapi_config_cloud,
		&build_id.to_string(),
		models::ActorPatchBuildTagsRequest {
			tags: Some(serde_json::to_value(&tags)?),
			exclusive_tags: Some(exclusive_tags.clone()),
		},
		Some(&ctx.project.name_id),
		Some(&env.slug),
	)
	.await;
	if let Err(err) = complete_res.as_ref() {
		task.log(format!("{err:?}"));
	}
	complete_res.context("complete_res")?;

	task.log(format!("[Build Upload Complete] {build_id}"));

	Ok(build_id)
}
