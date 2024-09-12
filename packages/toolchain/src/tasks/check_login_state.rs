use anyhow::*;
use serde::{Deserialize, Serialize};

use crate::{config, paths, util::task};

#[derive(Deserialize)]
pub struct Input {}

#[derive(Serialize)]
pub struct Output {
	pub logged_in: bool,
}

pub struct Task;

impl task::Task for Task {
	type Input = Input;
	type Output = Output;

	fn name() -> &'static str {
		"check_login_state"
	}

	async fn run(_task: task::TaskCtx, _input: Input) -> Result<Output> {
		let logged_in = config::meta::has_project(&paths::data_dir()?).await?;
		Ok(Output { logged_in })
	}
}
