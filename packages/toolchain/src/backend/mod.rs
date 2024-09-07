pub mod database;
pub mod embed;

use global_error::prelude::*;
use rivet_api::{apis, models};
use serde_json::json;
use std::{collections::HashMap, path::PathBuf};
use tokio::process::Command;
use uuid::Uuid;

use crate::{
	config,
	util::{cmd::shell_cmd, task},
	ToolchainCtx,
};

const OPENGB_DENO_CONFIG_PATH: &'static str = "/deno.jsonc";
const OPENGB_CLI_MAIN_PATH: &'static str = "/packages/cli/main.ts";

pub struct BackendCommandOpts {
	pub config_path: String,
	pub args: Vec<String>,
	pub env: HashMap<String, String>,
	pub cwd: PathBuf,
	pub ports: Vec<(u16, u16)>,
	pub enable_postgres: bool,
}

async fn base_url() -> GlobalResult<String> {
	// Attempt to read from user or default
	let base_url = if let Some(url) =
		config::settings::try_read(|x| Ok(x.backend.opengb_url.clone())).await?
	{
		url
	} else {
		embed::backend_dir().await?.display().to_string()
	};

	let base_url = base_url.trim_end_matches('/').to_string();
	Ok(base_url)
}

pub async fn build_opengb_command(opts: BackendCommandOpts) -> GlobalResult<Command> {
	let base_url = base_url().await?;

	// Download config from remote if needed.
	//
	// Deno does not support pulling config from URL.
	let deno_config_path = if base_url.starts_with("http://") || base_url.starts_with("https://") {
		let temp_dir = tempfile::tempdir()?.into_path();
		let deno_config_path = temp_dir.join("deno.jsonc");
		let deno_config_url = format!("{base_url}{OPENGB_DENO_CONFIG_PATH}");
		let response = reqwest::get(&deno_config_url).await?.error_for_status()?;
		let deno_config_content = response.text().await?;
		tokio::fs::write(&deno_config_path, deno_config_content).await?;
		deno_config_path.to_str().unwrap().to_string()
	} else {
		format!("{base_url}{OPENGB_DENO_CONFIG_PATH}")
	};

	// Get Deno executable
	let deno = crate::util::deno::get_or_download_executable(
		crate::util::deno::DEFAULT_VERSION,
		&crate::paths::data_dir()?,
	)
	.await?;

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
		&deno_config_path,
		&format!("{base_url}{OPENGB_CLI_MAIN_PATH}"),
		"--project",
		&opts.config_path,
	]);
	cmd.args(opts.args);
	cmd.envs(opts.env);
	cmd.current_dir(opts.cwd);
	Ok(cmd)
}

pub async fn run_opengb_command(
	task: task::TaskCtx,
	opts: BackendCommandOpts,
) -> GlobalResult<i32> {
	let cmd = build_opengb_command(opts).await?;
	let exit_code = task.spawn_cmd(cmd).await?;
	Ok(exit_code.code().unwrap_or(0))
}

pub async fn spawn_opengb_command(opts: BackendCommandOpts) -> GlobalResult<u32> {
	let child = build_opengb_command(opts).await?.spawn()?;
	Ok(unwrap!(child.id(), "child already exited"))
}

/// Gets or auto-creates a backend project for the game.
pub async fn get_or_create_backend(
	ctx: &ToolchainCtx,
	env_id: Uuid,
) -> GlobalResult<models::EeBackendBackend> {
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

async fn create_backend(
	ctx: &ToolchainCtx,
	env_id: Uuid,
) -> GlobalResult<models::EeBackendBackend> {
	let res = apis::ee_backend_api::ee_backend_create(
		&ctx.openapi_config_cloud,
		&ctx.game_id.to_string(),
		&env_id.to_string(),
		json!({}),
	)
	.await?;

	Ok(*res.backend)
}
