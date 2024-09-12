use anyhow::*;
use serde::{Deserialize, Serialize};

use crate::{paths, util::task};

#[derive(Deserialize)]
pub struct Input {}

#[derive(Serialize)]
pub struct Output {}

pub struct Task;

impl task::Task for Task {
	type Input = Input;
	type Output = Output;

	fn name() -> &'static str {
		"game_server_stop"
	}

	async fn run(_task: task::TaskCtx, _input: Self::Input) -> Result<Self::Output> {
		crate::game_server::PROCESS_MANAGER
			.stop(&paths::data_dir()?)
			.await?;
		Ok(Output {})
	}
}
