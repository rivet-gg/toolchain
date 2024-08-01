use global_error::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{config, util::task::TaskCtx};

// use crate::commands;

#[derive(Deserialize)]
pub struct Input {}

#[derive(Serialize)]
pub struct Output {}

pub struct Task;

impl super::Task for Task {
	type Input = Input;
	type Output = Output;

	fn name() -> &'static str {
		"unlink"
	}

	async fn run(_task: TaskCtx, _input: Self::Input) -> GlobalResult<Self::Output> {
		config::meta::delete_project().await?;
		Ok(Output {})
	}
}
