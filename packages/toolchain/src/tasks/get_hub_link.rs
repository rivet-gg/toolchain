use global_error::prelude::*;
use serde::{Deserialize, Serialize};

use crate::util::task;

#[derive(Deserialize)]
pub struct Input {
	pub kind: String,
}

#[derive(Serialize)]
pub struct Output {
	pub url: String,
}

pub struct Task;

impl task::Task for Task {
	type Input = Input;
	type Output = Output;

	fn name() -> &'static str {
		"get_hub_link"
	}

	async fn run(_task: task::TaskCtx, _input: Self::Input) -> GlobalResult<Self::Output> {
		bail!("todo")
	}
}
