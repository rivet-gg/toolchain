use std::collections::HashMap;

use anyhow::*;
use serde::{Deserialize, Serialize};

use crate::util::task;

#[derive(Deserialize)]
pub struct Input {}

#[derive(Serialize)]
pub struct Output {
	exit_code: Option<i32>,
}

pub struct Task;

impl task::Task for Task {
	type Input = Input;
	type Output = Output;

	fn name() -> &'static str {
		"game_server.hook"
	}

	async fn run(task: task::TaskCtx, _input: Self::Input) -> Result<Self::Output> {
		let exit_code = crate::game_server::PROCESS_MANAGER
			.hook(task.clone())
			.await?;
		Ok(Output { exit_code })
	}
}
