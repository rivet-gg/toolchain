pub mod database;

use anyhow::*;
use rivet_api::{apis, models};
use serde::Serialize;
use serde_json::json;
use std::{collections::HashMap, path::PathBuf, process::ExitCode, time::Duration};
use tokio::process::Command;
use uuid::Uuid;

use crate::{
	config, paths,
	util::{process_manager::ProcessManager, task},
	ToolchainCtx,
};

pub const PROCESS_MANAGER_DEV: ProcessManager = ProcessManager {
	key: "backend_dev",
	kill_grace: Duration::from_secs(2),
};

pub struct BackendCommandOpts {
	pub command: &'static str,
	pub opts: serde_json::Value,
	pub env: HashMap<String, String>,
}

async fn base_url() -> Result<String> {
	// Attempt to read from user or default
	let base_url = if let Some(url) =
		config::settings::try_read(&paths::data_dir()?, |x| Ok(x.backend.source_path.clone()))
			.await?
	{
		url
	} else {
		rivet_backend_embed::backend_dir(&paths::data_dir()?)
			.await?
			.display()
			.to_string()
	};

	let base_url = base_url.trim_end_matches('/').to_string();
	Ok(base_url)
}

pub struct CommandRaw {
	pub command: PathBuf,
	pub args: Vec<String>,
	pub envs: HashMap<String, String>,
	pub current_dir: PathBuf,
}

pub async fn build_backend_command_raw(opts: BackendCommandOpts) -> Result<CommandRaw> {
	let base_url = base_url().await?;

	// Get Deno executable
	let deno = rivet_deno_embed::get_or_download_executable(&crate::paths::data_dir()?).await?;

	// Serialize command
	let backend_cmd = serde_json::to_string(&json!({
		opts.command: opts.opts
	}))?;

	// Run backend
	Ok(CommandRaw {
		command: deno.executable_path,
		args: vec![
			"run".into(),
			"--quiet".into(),
			"--no-check".into(),
			"--allow-net".into(),
			"--allow-read".into(),
			"--allow-env".into(),
			"--allow-run".into(),
			"--allow-write".into(),
			"--allow-sys".into(),
			"--config".into(),
			format!("{base_url}/deno.jsonc"),
			"--lock".into(),
			format!("{base_url}/deno.lock"),
			format!("{base_url}/cli/main.ts"),
			"--command".into(),
			backend_cmd,
		],
		envs: opts.env,
		current_dir: paths::project_root()?,
	})
}

pub async fn build_backend_command(opts: BackendCommandOpts) -> Result<Command> {
	let cmd_raw = build_backend_command_raw(opts).await?;
	let mut cmd = Command::new(cmd_raw.command);
	cmd.args(cmd_raw.args)
		.envs(cmd_raw.envs)
		.current_dir(cmd_raw.current_dir);

	Ok(cmd)
}

pub async fn run_backend_command_from_task(
	task: task::TaskCtx,
	opts: BackendCommandOpts,
) -> Result<i32> {
	let cmd = build_backend_command(opts).await?;
	let exit_code = task.spawn_cmd(cmd).await?;
	Ok(exit_code.code().unwrap_or(0))
}

pub async fn run_backend_command_passthrough(
	command: &'static str,
	opts: &impl Serialize,
) -> ExitCode {
	let opts_json = match serde_json::to_value(opts) {
		Result::Ok(x) => x,
		Err(err) => {
			eprintln!("Serialize failed: {err:?}");
			return ExitCode::FAILURE;
		}
	};

	let mut cmd = match build_backend_command(BackendCommandOpts {
		command,
		opts: opts_json,
		env: HashMap::new(),
	})
	.await
	{
		Result::Ok(x) => x,
		Err(err) => {
			eprintln!("Error building command: {err:?}");
			return ExitCode::FAILURE;
		}
	};

	let exit_code = match cmd.status().await {
		Result::Ok(x) => x,
		Err(err) => {
			eprintln!("Error running command: {err:?}");
			return ExitCode::FAILURE;
		}
	};

	if exit_code.success() {
		ExitCode::SUCCESS
	} else {
		ExitCode::FAILURE
	}
}

/// Gets or auto-creates a backend project for the game.
pub async fn get_or_create_backend(
	ctx: &ToolchainCtx,
	env_id: Uuid,
) -> Result<models::EeBackendBackend> {
	// Get the project
	let backend_res = apis::ee_backend_api::ee_backend_get(
		&ctx.openapi_config_cloud,
		&ctx.game_id.to_string(),
		&env_id.to_string(),
		None,
	)
	.await;

	let backend = match backend_res {
		Err(apis::Error::ResponseError(apis::ResponseContent {
			entity:
				Some(apis::ee_backend_api::EeBackendGetError::Status400(models::ErrorBody {
					code, ..
				})),
			..
		})) if code == "BACKEND_NOT_FOUND" => create_backend(ctx, env_id).await?,
		x => *x?.backend,
	};

	Ok(backend)
}

async fn create_backend(ctx: &ToolchainCtx, env_id: Uuid) -> Result<models::EeBackendBackend> {
	let res = apis::ee_backend_api::ee_backend_create(
		&ctx.openapi_config_cloud,
		&ctx.game_id.to_string(),
		&env_id.to_string(),
		json!({}),
	)
	.await?;

	Ok(*res.backend)
}
