use anyhow::*;
use serde::Serialize;
use std::{collections::HashMap, path::PathBuf, process::ExitCode};
use tokio::process::Command;

use crate::{paths, util::task};

pub struct CommandOpts {
	pub task_path: &'static str,
	pub input: serde_json::Value,
	pub env: HashMap<String, String>,
}

async fn base_url() -> Result<String> {
	// Attempt to read from user or default
	let base_url = if let Some(url) = std::env::var("_RIVET_JS_UTILS_SRC_DIR").ok() {
		url
	} else {
		rivet_js_utils_embed::src_path(&paths::data_dir()?)
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

pub async fn build_backend_command_raw(opts: CommandOpts) -> Result<CommandRaw> {
	let base_url = base_url().await?;

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

pub async fn build_backend_command(opts: CommandOpts) -> Result<Command> {
	let cmd_raw = build_backend_command_raw(opts).await?;
	let mut cmd = Command::new(cmd_raw.command);
	cmd.args(cmd_raw.args)
		.envs(cmd_raw.envs)
		.current_dir(cmd_raw.current_dir);

	Ok(cmd)
}

pub async fn run_backend_command_from_task(task: task::TaskCtx, opts: CommandOpts) -> Result<i32> {
	let cmd = build_backend_command(opts).await?;
	let exit_code = task.spawn_cmd(cmd).await?;
	Ok(exit_code.code().unwrap_or(0))
}

pub async fn run_backend_command_passthrough(
	task_path: &'static str,
	input: &impl Serialize,
) -> ExitCode {
	let input_json = match serde_json::to_value(input) {
		Result::Ok(x) => x,
		Err(err) => {
			eprintln!("Serialize failed: {err:?}");
			return ExitCode::FAILURE;
		}
	};

	let mut cmd = match build_backend_command(CommandOpts {
		task_path,
		input: input_json,
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
