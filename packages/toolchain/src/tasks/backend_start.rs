use anyhow::*;
use serde::{Deserialize, Serialize};

use crate::{
	backend::{self, build_opengb_command_raw},
	config, paths,
	util::{process_manager::StartOpts, task},
};

#[derive(Deserialize)]
pub struct Input {
	pub port: u16,
	pub cwd: String,
}

#[derive(Serialize)]
pub struct Output {
	pub exit_code: Option<i32>,
}

pub struct Task;

impl task::Task for Task {
	type Input = Input;
	type Output = Output;

	fn name() -> &'static str {
		"backend_start"
	}

	async fn run(task: task::TaskCtx, input: Self::Input) -> Result<Self::Output> {
		let (mut cmd_env, config_path) =
			config::settings::try_read(&paths::data_dir()?, |settings| {
				let mut env = settings.backend.command_environment.clone();
				env.extend(settings.backend.dev.command_environment.clone());
				Ok((env, settings.backend.dev.config_path.clone()))
			})
			.await?;
		cmd_env.insert("OPENGB_PORT".into(), input.port.to_string());
		cmd_env.insert("OPENGB_HOSTNAME".into(), "0.0.0.0".to_string());
		cmd_env.insert("OPENGB_TERM_COLOR".into(), "never".into());

		// TODO: Does not support env
		let cmd = build_opengb_command_raw(backend::BackendCommandOpts {
			command: "dev",
			opts: serde_json::json!({
				"project": config_path,
				"nonInteractive": true
			}),
			env: cmd_env,
		})
		.await?;
		let exit_code = backend::PROCESS_MANAGER_DEV
			.start(StartOpts {
				task: task,
				command: cmd.command.display().to_string(),
				args: cmd.args,
				envs: cmd.envs.into_iter().collect(),
				current_dir: cmd.current_dir.display().to_string(),
				base_data_dir: paths::data_dir()?,
			})
			.await?;

		Ok(Output { exit_code })
	}
}
