use anyhow::*;
use serde::{Deserialize, Serialize};

use crate::{paths, postgres, util::task};

#[derive(Deserialize)]
pub struct Input {}

#[derive(Serialize)]
pub struct Output {}

pub struct Task;

impl task::Task for Task {
	type Input = Input;
	type Output = Output;

	fn name() -> &'static str {
		"postgres.reset"
	}

	async fn run(_task: task::TaskCtx, _input: Self::Input) -> Result<Self::Output> {
		postgres::get(&paths::data_dir()?).await?.reset().await?;
		Ok(Output {})
	}
}
