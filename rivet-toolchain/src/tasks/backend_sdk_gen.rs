use global_error::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{backend, util::task::TaskCtx};

#[derive(Deserialize)]
pub struct Input {
	pub cwd: String,
	pub output_path: String,
	pub target: String,
}

#[derive(Serialize)]
pub struct Output {
	pub exit_code: i32,
}

pub struct Task;

impl super::Task for Task {
	type Input = Input;
	type Output = Output;

	fn name() -> &'static str {
		"backend_sdk_gen"
	}

	async fn run(task: TaskCtx, input: Input) -> GlobalResult<Output> {
		let args = vec![
			"sdk".into(),
			"generate".into(),
			"--output".into(),
			input.output_path,
			input.target,
		];
		let mut env = HashMap::<String, String>::new();
		env.insert("OPENGB_TERM_COLOR".into(), "never".into());
		let exit_code = backend::run_opengb_command(
			task.clone(),
			backend::OpenGbCommandOpts {
				opengb_target: backend::OpenGbTarget::Native,
				args,
				env: HashMap::new(),
				cwd: input.cwd.into(),
			},
		)
		.await?;

		Ok(Output { exit_code })
	}
}
