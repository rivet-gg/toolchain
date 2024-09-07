use anyhow::*;
use serde::{Deserialize, Serialize};

use crate::util::task;

#[derive(Deserialize)]
pub struct Input {}

#[derive(Serialize)]
pub struct Output {
	pub port: u16,
}

pub struct Task;

impl task::Task for Task {
	type Input = Input;
	type Output = Output;

	fn name() -> &'static str {
		"backend_choose_local_port"
	}

	async fn run(_task: task::TaskCtx, _input: Self::Input) -> Result<Self::Output> {
		let port = portpicker::pick_unused_port().context("no free ports")?;
		Ok(Output { port })
	}
}
