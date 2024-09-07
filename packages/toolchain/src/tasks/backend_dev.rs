use anyhow::*;
use serde::{Deserialize, Serialize};

use crate::{backend, config, util::task};

#[derive(Deserialize)]
pub struct Input {
	pub port: u16,
	pub cwd: String,
}

#[derive(Serialize)]
pub struct Output {
	pub exit_code: i32,
}

pub struct Task;

impl task::Task for Task {
	type Input = Input;
	type Output = Output;

	fn name() -> &'static str {
		"backend_dev"
	}

	async fn run(task: task::TaskCtx, input: Self::Input) -> Result<Self::Output> {
		let (mut cmd_env, config_path) = config::settings::try_read(|settings| {
			let mut env = settings.backend.command_environment.clone();
			env.extend(settings.backend.dev.command_environment.clone());
			Ok((env, settings.backend.dev.config_path.clone()))
		})
		.await?;
		cmd_env.insert("OPENGB_PORT".into(), input.port.to_string());
		cmd_env.insert("OPENGB_HOSTNAME".into(), "0.0.0.0".to_string());
		cmd_env.insert("OPENGB_TERM_COLOR".into(), "never".into());
		let exit_code = backend::run_opengb_command_from_task(
			task.clone(),
			backend::BackendCommandOpts {
				command: "dev",
				opts: serde_json::json!({
					"project": config_path,
					"nonInteractive": true
				}),
				env: cmd_env,
			},
		)
		.await?;

		Ok(Output { exit_code })
	}
}
