use anyhow::*;
use serde::{Deserialize, Serialize};

use crate::{config, util::task};

#[derive(Deserialize)]
pub struct Input {}

#[derive(Serialize)]
pub struct Output {}

pub struct Task;

impl task::Task for Task {
	type Input = Input;
	type Output = Output;

	fn name() -> &'static str {
		"unlink"
	}

	async fn run(_task: task::TaskCtx, _input: Self::Input) -> Result<Self::Output> {
		config::meta::delete_project().await?;
		Ok(Output {})
	}
}
