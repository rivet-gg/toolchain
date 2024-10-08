use std::collections::HashMap;

use anyhow::*;
use serde::{Deserialize, Serialize};

use crate::util::{process_manager::CommandOpts, task};

#[derive(Deserialize)]
pub struct Input {
	pub cmd: String,
	pub args: Vec<String>,
	pub cwd: String,
	pub envs: HashMap<String, String>,
}

#[derive(Serialize)]
pub struct Output {
	exit_code: Option<i32>,
}

pub struct Task;

impl task::Task for Task {
	type Input = Input;
	type Output = Output;

	fn name() -> &'static str {
		"game_server.start"
	}

	async fn run(task: task::TaskCtx, input: Self::Input) -> Result<Self::Output> {
		let exit_code = crate::game_server::PROCESS_MANAGER
			.start(task.clone(), move || async move {
				Ok(CommandOpts {
					command: input.cmd,
					args: input.args,
					envs: input.envs.into_iter().collect(),
					current_dir: input.cwd,
				})
			})
			.await?;
		Ok(Output { exit_code })
	}
}
