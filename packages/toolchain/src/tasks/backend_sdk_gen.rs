use anyhow::*;
use serde::{Deserialize, Serialize};

use crate::{backend, config, util::task};

#[derive(Deserialize)]
pub struct Input {
	pub cwd: String,
	/// If a path is not provided in settings, use this.
	pub fallback_sdk_path: String,
	pub target: String,
}

#[derive(Serialize)]
pub struct Output {
	pub exit_code: i32,
	pub sdk_path: String,
}

pub struct Task;

impl task::Task for Task {
	type Input = Input;
	type Output = Output;

	fn name() -> &'static str {
		"backend_sdk_gen"
	}

	async fn run(task: task::TaskCtx, input: Input) -> Result<Output> {
		let (mut cmd_env, sdk_settings, config_path) = config::settings::try_read(|settings| {
			let mut env = settings.backend.command_environment.clone();
			env.extend(settings.backend.sdk.command_environment.clone());
			Ok((
				env,
				settings.backend.sdk.clone(),
				settings.backend.deploy.config_path.clone(),
			))
		})
		.await?;

		let sdk_path = sdk_settings
			.path
			.unwrap_or_else(|| input.fallback_sdk_path.clone());

		cmd_env.insert("OPENGB_TERM_COLOR".into(), "never".into());

		let exit_code = backend::run_opengb_command_from_task(
			task.clone(),
			backend::BackendCommandOpts {
				command: "sdkGenerate",
				opts: serde_json::json!({
					"project": config_path,
					"target": input.target,
					"output": sdk_path,
				}),
				env: cmd_env,
			},
		)
		.await?;

		Ok(Output {
			exit_code,
			sdk_path,
		})
	}
}
