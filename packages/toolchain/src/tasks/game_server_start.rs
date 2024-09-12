use anyhow::*;
use serde::{Deserialize, Serialize};

use crate::{
	paths,
	util::{
		process_manager::{CommandOpts, StartMode, StartOpts},
		task,
	},
};

#[derive(Deserialize)]
pub struct Input {
	pub start_mode: StartMode,
	pub cmd: String,
	pub args: Vec<String>,
	pub cwd: String,
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
		"game_server_start"
	}

	async fn run(task: task::TaskCtx, input: Self::Input) -> Result<Self::Output> {
		let exit_code = crate::game_server::PROCESS_MANAGER
			.start(
				StartOpts {
					task: task.clone(),
					base_data_dir: paths::data_dir()?,
					start_mode: input.start_mode,
				},
				move || async move {
					Ok(CommandOpts {
						command: input.cmd,
						args: input.args,
						envs: Vec::new(),
						current_dir: input.cwd,
					})
				},
			)
			.await?;
		Ok(Output { exit_code })
	}
}
