use global_error::prelude::*;
use serde::{Deserialize, Serialize};

use crate::util::task::TaskCtx;

#[derive(Deserialize)]
pub struct Input {}

#[derive(Serialize)]
pub struct Output {
	pub port: u16,
}

pub struct Task;

impl super::Task for Task {
	type Input = Input;
	type Output = Output;

	fn name() -> &'static str {
		"backend_choose_local_port"
	}

	async fn run(_task: TaskCtx, _input: Self::Input) -> GlobalResult<Self::Output> {
		let port = unwrap!(portpicker::pick_unused_port(), "no free ports");
		Ok(Output { port })
	}
}
