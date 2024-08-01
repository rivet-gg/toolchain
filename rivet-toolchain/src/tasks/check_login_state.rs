use global_error::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{config, util::task::TaskCtx};

#[derive(Deserialize)]
pub struct Input {}

#[derive(Serialize)]
pub struct Output {}

pub struct Task;

impl super::Task for Task {
	type Input = Input;
	type Output = Output;

	fn name() -> &'static str {
		"check_login_state"
	}

	async fn run(task: TaskCtx, input: Input) -> GlobalResult<Output> {
		let has_token = config::global::read_project(|x| x.tokens.cloud.is_some()).await?;
		if !has_token {
			bail!("No Rivet token found, please do the sign in process");
		}

		Ok(Output {})
	}
}
