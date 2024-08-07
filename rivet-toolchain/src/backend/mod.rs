pub mod database;

use global_error::prelude::*;
use rivet_api::{apis, models};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::Write, path::PathBuf};
use tempfile::NamedTempFile;
use tokio::process::Command;

use crate::{
	config,
	util::{cmd::shell_cmd, task::TaskCtx},
	Ctx,
};

const DEFAULT_OPENGB_DOCKER_TAG: &'static str = "ghcr.io/rivet-gg/opengb/v0.1.2";
pub struct OpenGbCommandOpts {
	pub config_path: String,
	pub args: Vec<String>,
	pub env: HashMap<String, String>,
	pub cwd: PathBuf,
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

pub async fn build_opengb_command(opts: OpenGbCommandOpts) -> GlobalResult<Command> {
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
			cmd.arg("--");
			cmd.arg("--project");
			cmd.arg(opts.config_path);
			cmd.args(&opts.args);
			Ok(cmd)
		}
	}
}

pub async fn run_opengb_command(task: TaskCtx, opts: OpenGbCommandOpts) -> GlobalResult<i32> {
	let cmd = build_opengb_command(opts).await?;
	let exit_code = task.spawn_cmd(cmd).await?;
	Ok(exit_code.code().unwrap_or(0))
}

pub async fn spawn_opengb_command(opts: OpenGbCommandOpts) -> GlobalResult<u32> {
	let child = build_opengb_command(opts).await?.spawn()?;
	Ok(unwrap!(child.id(), "child already exited"))
}

/// Gets or auto-creates a backend project for the game.
pub async fn get_or_create_project(ctx: &Ctx) -> GlobalResult<Box<models::EeBackendProject>> {
	// Get the project
	let project_res = apis::ee_cloud_games_projects_api::ee_cloud_games_projects_get(
		&ctx.openapi_config_cloud,
		&ctx.game_id,
	)
	.await?;

	if let Some(project) = project_res.project {
		Ok(project)
	} else {
		// Get game in order to determine team & project name
		let models::CloudGamesGetGameByIdResponse { game, .. } =
			apis::cloud_games_api::cloud_games_get_game_by_id(
				&ctx.openapi_config_cloud,
				&ctx.game_id,
				None,
			)
			.await?;

		// Create project
		let display_name = format!("{:.15}-backend", game.display_name);
		let models::EeCloudBackendProjectsCreateResponse { project, .. } =
			apis::ee_cloud_backend_projects_api::ee_cloud_backend_projects_create(
				&ctx.openapi_config_cloud,
				models::EeCloudBackendProjectsCreateRequest {
					developer_group_id: game.developer_group_id,
					display_name,
					game_id: Some(game.game_id),
				},
			)
			.await?;

		// Link to game
		apis::ee_cloud_games_projects_api::ee_cloud_games_projects_link(
			&ctx.openapi_config_cloud,
			&ctx.game_id,
			models::EeCloudGamesProjectsLinkRequest {
				project_id: project.project_id,
			},
		)
		.await?;

		Ok(project)
	}
}
