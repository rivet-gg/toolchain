use global_error::prelude::*;
use serde::{Deserialize, Serialize};

use crate::util::task::TaskCtx;

#[derive(Deserialize)]
pub struct Input {}

#[derive(Serialize)]
pub struct Output {
	pub token: String,
	pub api_endpoint: String,
	pub game_id: String,
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
		Ok(Output {
			token: ctx.access_token.clone(),
			api_endpoint: ctx.api_endpoint.clone(),
			game_id: ctx.game_id.clone(),
		})
	}
}
