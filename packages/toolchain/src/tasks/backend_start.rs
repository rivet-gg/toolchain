use anyhow::*;
use serde::{Deserialize, Serialize};

use crate::{
	backend::{self, build_opengb_command_raw},
	config::{self, meta},
	paths,
	util::{
		process_manager::{CommandOpts, StartMode, StartOpts},
		task,
	},
};

#[derive(Deserialize)]
pub struct Input {
	pub start_mode: StartMode,
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
		// HACK: Set backend port in case the process is already running. This will result in a
		// duplicate port dispatch if the backend was stopped.
		if let Some(port) = meta::read_project(&paths::data_dir()?, |x| x.backend_dev_port).await? {
			task.event(task::TaskEvent::SetBackendPort { port });
		}

		// Start or hook to backend
		let task_inner = task.clone();
		let exit_code = backend::PROCESS_MANAGER_DEV
			.start(
				StartOpts {
					task,
					start_mode: input.start_mode,
					base_data_dir: paths::data_dir()?,
				},
				|| async move {
					// Pick dev port
					let port = portpicker::pick_unused_port().context("no free ports")?;
					meta::mutate_project(&paths::data_dir()?, |x| x.backend_dev_port = Some(port))
						.await?;

					// Build env
					let (mut cmd_env, config_path) =
						config::settings::try_read(&paths::data_dir()?, |settings| {
							let mut env = settings.backend.command_environment.clone();
							env.extend(settings.backend.dev.command_environment.clone());
							Ok((env, settings.backend.dev.config_path.clone()))
						})
						.await?;
					cmd_env.insert("OPENGB_PORT".into(), port.to_string());
					cmd_env.insert("OPENGB_HOSTNAME".into(), "0.0.0.0".to_string());
					cmd_env.insert("OPENGB_TERM_COLOR".into(), "never".into());

					// Build command
					let cmd = build_opengb_command_raw(backend::BackendCommandOpts {
						command: "dev",
						opts: serde_json::json!({
							"project": config_path,
							"nonInteractive": true
						}),
						env: cmd_env,
					})
					.await?;

					// Publish commandevent
					task_inner.event(task::TaskEvent::SetBackendPort { port });

					Ok(CommandOpts {
						command: cmd.command.display().to_string(),
						args: cmd.args,
						envs: cmd.envs.into_iter().collect(),
						current_dir: cmd.current_dir.display().to_string(),
					})
				},
			)
			.await?;

		Ok(Output { exit_code })
	}
}
