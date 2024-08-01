use global_error::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{backend, config, util::task::TaskCtx};

#[derive(Deserialize)]
pub struct Input {
	pub cwd: String,
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
		"backend_dev"
	}

	async fn run(task: TaskCtx, input: Self::Input) -> GlobalResult<Self::Output> {
		let mut cmd_env = config::settings::try_read(|settings| {
			let mut env = settings.backend.command_environment.clone();
			env.extend(settings.backend.dev.command_environment.clone());
			Ok(env)
		})
		.await?;
		cmd_env.insert("OPENGB_TERM_COLOR".into(), "never".into());
		let exit_code = backend::run_opengb_command(
			task.clone(),
			backend::OpenGbCommandOpts {
				args: vec!["dev".into(), "--force-deploy-migrations".into()],
				env: cmd_env,
				cwd: input.cwd.into(),
			},
		)
		.await?;

		Ok(Output { exit_code })
	}
}
