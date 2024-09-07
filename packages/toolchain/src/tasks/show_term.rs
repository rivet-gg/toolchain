use anyhow::*;
use serde::{Deserialize, Serialize};

use crate::{util, util::task};

#[derive(Deserialize)]
pub struct Input {
	pub command: String,
	pub args: Vec<String>,
}

#[derive(Serialize)]
pub struct Output {}

pub struct Task;

impl task::Task for Task {
	type Input = Input;
	type Output = Output;

	fn name() -> &'static str {
		"show_term"
	}

	async fn run(_task: task::TaskCtx, input: Self::Input) -> Result<Self::Output> {
		let mut command = Vec::new();
		command.push(input.command);
		command.extend(input.args);
		util::show_term::show_term(&command).await?;

		Ok(Output {})
	}
}
