use anyhow::*;
use serde::{Deserialize, Serialize};

use crate::util::{cmd::shell_cmd, task};

#[derive(Deserialize)]
pub struct Input {
	pub cwd: String,
	pub cmd: String,
	pub args: Vec<String>,
}

#[derive(Serialize)]
pub struct Output {
	exit_code: i32,
}

pub struct Task;

impl task::Task for Task {
	type Input = Input;
	type Output = Output;

	fn name() -> &'static str {
		"exec_command"
	}

	async fn run(task: task::TaskCtx, input: Self::Input) -> Result<Self::Output> {
		let mut cmd = shell_cmd(&input.cmd);
		cmd.args(&input.args).current_dir(input.cwd);
		let exit_code = task.spawn_cmd(cmd).await?;
		Ok(Output {
			exit_code: exit_code.code().unwrap_or(0),
		})
	}
}
