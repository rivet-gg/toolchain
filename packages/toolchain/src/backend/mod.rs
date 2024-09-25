pub mod database;

use anyhow::*;
use lazy_static::lazy_static;
use rivet_api::{apis, models};
use serde::Serialize;
use serde_json::json;
use std::{collections::HashMap, path::PathBuf, process::ExitCode, sync::Arc, time::Duration};
use tokio::process::Command;
use uuid::Uuid;

use crate::{
	config,
	paths::{self, BackendDataType},
	postgres,
	util::{process_manager::ProcessManager, task},
	ToolchainCtx,
};

lazy_static! {
	pub static ref PROCESS_MANAGER_DEV: Arc<ProcessManager> =
		ProcessManager::new("backend_dev", Duration::from_secs(2));
}

pub struct BackendCommandOpts {
	pub task_path: &'static str,
	pub input: serde_json::Value,
	pub env: HashMap<String, String>,
	pub data_type: BackendDataType,
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

pub async fn build_backend_command_raw(mut opts: BackendCommandOpts) -> Result<CommandRaw> {
	let base_url = base_url().await?;

	// Get data dir
	let backend_data_dir = paths::backend_data_dir(&paths::data_dir()?, opts.data_type)?;
	opts.env.insert(
		"BACKEND_DATA_DIR".into(),
		backend_data_dir.display().to_string(),
	);

	// Add development Postgres URL if not already specified
	//
	// When deploying, this will already be specified
	if !opts.env.contains_key("DATABASE_URL") {
		let postgres = postgres::get(&paths::data_dir()?).await?;
		let db_url = postgres.url("postgres").await;
		opts.env.insert("DATABASE_URL".into(), db_url);
	}

	// Get Deno executable
	let deno = rivet_deno_embed::get_executable(&crate::paths::data_dir()?).await?;

	// Serialize command
	let input_json = serde_json::to_string(&opts.input)?;

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
			format!("{base_url}/cli/tasks/{}", opts.task_path),
			"--input".into(),
			input_json,
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
	task_path: &'static str,
	input: &impl Serialize,
	data_type: BackendDataType,
) -> ExitCode {
	let input_json = match serde_json::to_value(input) {
		Result::Ok(x) => x,
		Err(err) => {
			eprintln!("Serialize failed: {err:?}");
			return ExitCode::FAILURE;
		}
	};

	let mut cmd = match build_backend_command(BackendCommandOpts {
		task_path,
		input: input_json,
		env: HashMap::new(),
		data_type,
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
