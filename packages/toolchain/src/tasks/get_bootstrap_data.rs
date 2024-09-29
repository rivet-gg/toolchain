use anyhow::*;
use futures_util::{StreamExt, TryStreamExt};
use rivet_api::{apis, models};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;

use crate::{backend, game::TEMPEnvironment, game_server, util::task};

#[derive(Deserialize)]
pub struct Input {}

#[derive(Serialize)]
pub struct Output {
	pub cloud: Option<CloudData>,
}

#[derive(Serialize)]
pub struct CloudData {
	pub token: String,
	pub api_endpoint: String,
	pub game_id: String,
	pub envs: Vec<TEMPEnvironment>,
	pub backends: HashMap<Uuid, models::EeBackendBackend>,
	pub current_builds: HashMap<Uuid, models::ServersBuild>,
}

pub struct Task;

impl task::Task for Task {
	type Input = Input;
	type Output = Output;

	fn name() -> &'static str {
		"get_bootstrap_data"
	}

	async fn run(_task: task::TaskCtx, _input: Self::Input) -> Result<Self::Output> {
		let cloud = if let Some(ctx) = crate::toolchain_ctx::try_load().await? {
			// HACK: Map ns to temporary env data structure
			// Get or create backend project
			let envs = apis::cloud_games_api::cloud_games_get_game_by_id(
				&ctx.openapi_config_cloud,
				&ctx.game_id.to_string(),
				None,
			)
			.await?
			.game
			.namespaces
			.into_iter()
			.map(crate::game::TEMPEnvironment::from)
			.collect::<Vec<_>>();

			// Get all backends in parallel
			let backends_fut = futures_util::stream::iter(envs.iter().cloned())
				.map({
					|env| {
						let ctx = ctx.clone();
						async move {
							let backend = backend::get_or_create_backend(&ctx, env.id).await?;
							Result::<_, anyhow::Error>::Ok((env.id, backend))
						}
					}
				})
				.buffer_unordered(4)
				.try_collect::<HashMap<Uuid, models::EeBackendBackend>>();

			// Get all current builds in parallel
			let current_builds_fut = futures_util::stream::iter(envs.iter().cloned())
				.map({
					|env| {
						let ctx = ctx.clone();
						async move {
							// Fetch build with the current tag & select first one
							let tags = serde_json::to_string(&json!({
								game_server::CURRENT_BUILD_TAG: "true"
							}))?;
							let current_build = apis::servers_builds_api::servers_builds_list(
								&ctx.openapi_config_cloud,
								&ctx.game_id.to_string(),
								&env.id.to_string(),
								Some(&tags),
							)
							.await?;
							if let Some(build) = current_build.builds.into_iter().next() {
								Result::<_, anyhow::Error>::Ok(Some((env.id, build)))
							} else {
								Result::<_, anyhow::Error>::Ok(None)
							}
						}
					}
				})
				.buffer_unordered(4)
				.try_collect::<Vec<Option<(Uuid, models::ServersBuild)>>>();

			// Query in parallel
			let (backends, current_builds) = tokio::try_join!(backends_fut, current_builds_fut)?;

			// Convert to map
			let current_builds = current_builds
				.into_iter()
				.filter_map(|x| x)
				.collect::<HashMap<Uuid, models::ServersBuild>>();

			Some(CloudData {
				token: ctx.access_token.clone(),
				api_endpoint: ctx.api_endpoint.clone(),
				game_id: ctx.game_id.clone(),
				envs,
				backends,
				current_builds,
			})
		} else {
			None
		};

		Ok(Output { cloud })
	}
}
