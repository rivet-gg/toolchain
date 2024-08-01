pub mod database;

use global_error::prelude::*;
use rivet_api::{apis, models};
use std::{collections::HashMap, fs::File, io::Write, path::PathBuf, str::FromStr};
use tempfile::NamedTempFile;
use tokio::{fs, process::Command, signal};

use crate::{
	util::{cmd::shell_cmd, task::TaskCtx},
	Ctx,
};

const DEFAULT_OPENGB_DOCKER_TAG: &'static str = "ghcr.io/rivet-gg/opengb/v0.1.2";
pub struct OpenGbCommandOpts {
	pub opengb_target: OpenGbTarget,
	pub args: Vec<String>,
	pub env: HashMap<String, String>,
	pub cwd: PathBuf,
}

#[derive(PartialEq)]
pub enum OpenGbTarget {
	Native,
	Docker,
}

impl Default for OpenGbTarget {
	fn default() -> Self {
		Self::Docker
	}
}
impl FromStr for OpenGbTarget {
	type Err = GlobalError;

	fn from_str(s: &str) -> GlobalResult<Self> {
		match s {
			"native" => Ok(Self::Native),
			"docker" => Ok(Self::Docker),
			_ => bail!("unknown opengb target: {s}"),
		}
	}
}

pub fn build_opengb_command(opts: OpenGbCommandOpts) -> GlobalResult<Command> {
	// Build command
	match opts.opengb_target {
		OpenGbTarget::Native => {
			let mut cmd = shell_cmd("opengb");
			cmd.args(opts.args);
			cmd.envs(opts.env);
			cmd.current_dir(opts.cwd);
			Ok(cmd)
		}
		OpenGbTarget::Docker => {
			let image_tag = std::env::var("RIVET_OPENGB_DOCKER_IMAGE")
				.ok()
				.unwrap_or_else(|| DEFAULT_OPENGB_DOCKER_TAG.to_string());

			// Build env file
			let mut env_file = NamedTempFile::new().expect("Failed to create temp file");
			for (k, v) in std::env::vars() {
				writeln!(env_file, "{k}={v}")?;
			}
			if std::env::var("DATABASE_URL").is_err() {
				writeln!(env_file, "DATABASE_URL=postgres://postgres:postgres@host.docker.internal:5432/postgres?sslmode=disable")?;
			}
			for (k, v) in opts.env {
				writeln!(env_file, "{k}={v}")?;
			}

			let mut cmd = shell_cmd("docker");
			cmd.arg("run");
			cmd.arg("--interactive");
			cmd.arg("--tty");
			cmd.arg("--quiet");
			cmd.arg("--init");
			cmd.arg("--env-file").arg(env_file.path());
			cmd.arg("--add-host=host.docker.internal:host-gateway");
			cmd.arg("--publish=6420:6420");
			cmd.arg(format!("--volume={}:/backend", opts.cwd.display()));
			cmd.arg("--workdir=/backend");
			cmd.arg(image_tag);
			cmd.args(&opts.args);
			Ok(cmd)
		}
	}
}

pub async fn run_opengb_command(task: TaskCtx, opts: OpenGbCommandOpts) -> GlobalResult<i32> {
	let cmd = build_opengb_command(opts)?;
	let exit_code = task.spawn_cmd(cmd).await?;
	Ok(exit_code.code().unwrap_or(0))
}

pub async fn spawn_opengb_command(opts: OpenGbCommandOpts) -> GlobalResult<u32> {
	let child = build_opengb_command(opts)?.spawn()?;
	Ok(unwrap!(child.id(), "child already exited"))
}

/// Gets or auto-creates a backend project for the game.
pub async fn get_or_create_project(ctx: &Ctx) -> GlobalResult<Box<models::EeBackendProject>> {
	let project_res = apis::ee_cloud_games_projects_api::ee_cloud_games_projects_get(
		&ctx.openapi_config_cloud,
		&ctx.game_id,
	)
	.await?;

	// TOOD: Add get or create project
	let project = unwrap!(
			project_res.project,
			"No OpenGB project linked to the current game. Create one on the hub: https://hub.rivet.gg/"
		);

	Ok(project)
}
