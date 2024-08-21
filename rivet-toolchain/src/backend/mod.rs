pub mod database;

use global_error::prelude::*;
use rivet_api::{apis, models};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, io::Write, path::PathBuf};
use tempfile::NamedTempFile;
use tokio::process::Command;
use uuid::Uuid;

use crate::{
	config,
	util::{cmd::shell_cmd, task::TaskCtx},
	Ctx,
};

const DEFAULT_OPENGB_DOCKER_TAG: &'static str = "ghcr.io/rivet-gg/opengb:main";

pub struct BackendCommandOpts {
	pub config_path: String,
	pub args: Vec<String>,
	pub env: HashMap<String, String>,
	pub cwd: PathBuf,
	pub ports: Vec<(u16, u16)>,
}

#[derive(PartialEq, Serialize, Deserialize, Clone)]
pub enum OpenGbRuntime {
	Native,
	Docker,
}

impl Default for OpenGbRuntime {
	fn default() -> Self {
		Self::Docker
	}
}

pub async fn build_opengb_command(opts: BackendCommandOpts) -> GlobalResult<Command> {
	let (runtime, image_tag) = config::settings::try_read(|settings| {
		Ok((
			settings.backend.opengb_runtime.clone(),
			settings.backend.opengb_docker_image.clone(),
		))
	})
	.await?;

	// Build command
	match runtime {
		OpenGbRuntime::Native => {
			let mut cmd = shell_cmd("opengb");
			cmd.arg("--project").arg(opts.config_path);
			cmd.args(opts.args);
			cmd.envs(opts.env);
			cmd.current_dir(opts.cwd);
			Ok(cmd)
		}
		OpenGbRuntime::Docker => {
			let image_tag = image_tag.unwrap_or_else(|| DEFAULT_OPENGB_DOCKER_TAG.to_string());

			// Build env file
			let mut env_file = NamedTempFile::new().expect("Failed to create temp file");
			for (k, v) in opts.env {
				writeln!(env_file, "{k}={v}")?;
			}
			// Make sure the file is properly flushed, and doesn't get deleted
			// after the NamedTempFile goes out of scope
			env_file.flush()?;
			let (_env_file, env_file_path) = env_file.keep()?;

			let mut cmd = shell_cmd("docker");
			cmd.arg("run");
			cmd.arg("--rm");
			cmd.arg("--interactive");
			cmd.arg("--quiet");
			cmd.arg("--init");
			cmd.arg("--env-file").arg(env_file_path);
			cmd.arg("--add-host=host.docker.internal:host-gateway");
			// Mount the project
			cmd.arg(format!("--volume={}:/backend", opts.cwd.display()));
			// Mount Postgres volume for bundled Postgres server
			cmd.arg("--volume=opengb_postgres:/var/lib/postgresql/data");
			cmd.arg("--workdir=/backend");
			for (host_port, container_port) in opts.ports {
				cmd.arg(format!("--publish={}:{}", host_port, container_port));
			}
			cmd.arg(image_tag);

			cmd.arg("--project");
			cmd.arg(opts.config_path);

			cmd.args(&opts.args);
			Ok(cmd)
		}
	}
}

pub async fn run_opengb_command(task: TaskCtx, opts: BackendCommandOpts) -> GlobalResult<i32> {
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
	ctx: &Ctx,
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

async fn create_backend(ctx: &Ctx, env_id: Uuid) -> GlobalResult<models::EeBackendBackend> {
	let res = apis::ee_backend_api::ee_backend_create(
		&ctx.openapi_config_cloud,
		&ctx.game_id.to_string(),
		&env_id.to_string(),
		json!({}),
	)
	.await?;

	Ok(*res.backend)
}
