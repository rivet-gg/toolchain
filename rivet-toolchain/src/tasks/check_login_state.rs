use global_error::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{config, util::task::TaskCtx};

#[derive(Deserialize)]
pub struct Input {}

#[derive(Serialize)]
pub struct Output {
	logged_in: bool,
}

pub struct Task;

impl super::Task for Task {
	type Input = Input;
	type Output = Output;

	fn name() -> &'static str {
		"check_login_state"
	}

	async fn run(_task: TaskCtx, _input: Input) -> GlobalResult<Output> {
		let logged_in = config::meta::has_project().await?;
		Ok(Output { logged_in })
	}
}
