use futures_util::stream::{StreamExt, TryStreamExt};
use global_error::prelude::*;
use rivet_api::{apis, models};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::{backend, game::TEMPEnvironment, util::task::TaskCtx};

#[derive(Deserialize)]
pub struct Input {}

#[derive(Serialize)]
pub struct Output {
	pub token: String,
	pub api_endpoint: String,
	pub game_id: String,
	pub envs: Vec<TEMPEnvironment>,
	pub backends: HashMap<Uuid, models::EeBackendBackend>,
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
		let backends = futures_util::stream::iter(envs.iter())
			.map(|env| {
				let ctx = ctx.clone();
				async move {
					let backend = backend::get_or_create_backend(&ctx, env.id).await?;
					GlobalResult::Ok((env.id, backend))
				}
			})
			.buffer_unordered(4)
			.try_collect::<HashMap<Uuid, models::EeBackendBackend>>()
			.await?;

		Ok(Output {
			token: ctx.access_token.clone(),
			api_endpoint: ctx.api_endpoint.clone(),
			game_id: ctx.game_id.clone(),
			envs,
			backends,
		})
	}
}
