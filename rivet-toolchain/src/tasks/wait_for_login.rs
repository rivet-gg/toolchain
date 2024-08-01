use global_error::prelude::*;
use rivet_api::apis;
use serde::{Deserialize, Serialize};

use crate::{config, ctx, util::task::TaskCtx};

#[derive(Deserialize)]
pub struct Input {
	pub device_link_token: String,
}

#[derive(Serialize)]
pub struct Output {
	pub output: String,
}

pub struct Task;

impl super::Task for Task {
	type Input = Input;
	type Output = Output;

	fn name() -> &'static str {
		"wait_for_login"
	}

	async fn run(task: TaskCtx, input: Self::Input) -> GlobalResult<Self::Output> {
		let (api_endpoint, _token) = config::global::read_project(|x| {
			(x.cluster.api_endpoint.clone(), x.tokens.cloud.clone())
		})
		.await?;

		let openapi_config_cloud_unauthed = apis::configuration::Configuration {
			base_path: api_endpoint
				.clone()
				.unwrap_or_else(|| ctx::DEFAULT_API_ENDPOINT.to_string()),
			user_agent: Some(ctx::user_agent()),
			..Default::default()
		};

		let mut watch_index = None;
		let token = loop {
			let prepare_res = apis::cloud_devices_links_api::cloud_devices_links_get(
				&openapi_config_cloud_unauthed,
				&input.device_link_token,
				watch_index.as_ref().map(String::as_str),
			)
			.await?;

			watch_index = Some(prepare_res.watch.index);

			if let Some(token) = prepare_res.cloud_token {
				break token;
			}
		};

		let new_ctx = crate::ctx::init(api_endpoint, token.clone()).await?;

		let inspect_res =
			apis::cloud_auth_api::cloud_auth_inspect(&new_ctx.openapi_config_cloud).await?;

		let game_id = unwrap!(inspect_res.agent.game_cloud, "no game cloud token found").game_id;

		let _game_res = apis::cloud_games_api::cloud_games_get_game_by_id(
			&new_ctx.openapi_config_cloud,
			&game_id.to_string(),
			None,
		)
		.await?;

		config::global::mutate_project(|x| x.tokens.cloud = Some(token)).await?;

		Ok(Output {
			output: "Token saved".to_string(),
		})
	}
}
