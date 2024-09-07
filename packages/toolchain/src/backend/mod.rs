pub mod database;

use anyhow::*;
use rivet_api::{apis, models};
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use std::process::ExitCode;
use tokio::process::Command;
use uuid::Uuid;

use crate::{
	config, paths,
	util::{cmd::shell_cmd, task},
	ToolchainCtx,
};

pub struct BackendCommandOpts {
	pub command: &'static str,
	pub opts: serde_json::Value,
	pub env: HashMap<String, String>,
}

async fn base_url() -> Result<String> {
	// Attempt to read from user or default
	let base_url = if let Some(url) =
		config::settings::try_read(|x| Ok(x.backend.source_path.clone())).await?
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

pub async fn build_opengb_command(opts: BackendCommandOpts) -> Result<Command> {
	let base_url = base_url().await?;

	// Get Deno executable
	let deno =
		rivet_deno_embed::get_or_download_default_executable(&crate::paths::data_dir()?).await?;

	// Serialize command
	let backend_cmd = &serde_json::to_string(&json!({
		opts.command: opts.opts
	}))?;

	// Run OpenGB
	let mut cmd = shell_cmd(&deno.executable_path.display().to_string());
	cmd.args(&[
		"run",
		"--quiet",
		"--no-check",
		"--allow-net",
		"--allow-read",
		"--allow-env",
		"--allow-run",
		"--allow-write",
		"--allow-sys",
		"--config",
		&format!("{base_url}/deno.jsonc"),
		"--lock",
		&format!("{base_url}/deno.lock"),
		&format!("{base_url}/cli/main.ts"),
		"--command",
		backend_cmd,
	]);
	cmd.envs(opts.env);
	cmd.current_dir(paths::project_root()?);
	Ok(cmd)
}

pub async fn run_opengb_command_from_task(
	task: task::TaskCtx,
	opts: BackendCommandOpts,
) -> Result<i32> {
	let cmd = build_opengb_command(opts).await?;
	let exit_code = task.spawn_cmd(cmd).await?;
	Ok(exit_code.code().unwrap_or(0))
}

pub async fn run_opengb_command(opts: BackendCommandOpts) -> Result<i32> {
	let mut cmd = build_opengb_command(opts).await?;
	let exit_code = cmd.status().await?;
	Ok(exit_code.code().unwrap_or(0))
}

pub async fn run_opengb_command_passthrough(
	command: &'static str,
	opts: &impl Serialize,
) -> ExitCode {
	let opts_json = match serde_json::to_value(opts) {
		Result::Ok(x) => x,
		Err(err) => {
			eprintln!("Serialize failed");
			return ExitCode::FAILURE;
		}
	};

	let mut cmd = match build_opengb_command(BackendCommandOpts {
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

pub async fn spawn_opengb_command(opts: BackendCommandOpts) -> Result<u32> {
	let child = build_opengb_command(opts).await?.spawn()?;
	Ok(child.id().context("child already exited")?)
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
