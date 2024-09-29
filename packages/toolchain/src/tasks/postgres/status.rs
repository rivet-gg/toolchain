use anyhow::*;
use serde::{Deserialize, Serialize};

use crate::{paths, postgres, util::task};

#[derive(Deserialize)]
pub struct Input {}

#[derive(Serialize)]
pub struct Output {
	pub status: postgres::Status,
}

pub struct Task;

impl task::Task for Task {
	type Input = Input;
	type Output = Output;

	fn name() -> &'static str {
		"postgres.status"
	}

	async fn run(_task: task::TaskCtx, _input: Self::Input) -> Result<Self::Output> {
		let status = postgres::get(&paths::data_dir()?).await?.status().await?;
		Ok(Output { status })
	}
}
