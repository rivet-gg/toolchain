use std::{collections::HashMap, io::Write, path::PathBuf};

use clap::Parser;
use cli_core::rivet_api::{apis, models};
use global_error::prelude::*;
use serde::Deserialize;
use tempfile::NamedTempFile;
use tokio::fs;
use tokio::process::Command;

use crate::util::{global_config, paths, term};

const DEFAULT_OPENGB_DOCKER_TAG: &'static str = "ghcr.io/rivet-gg/opengb/v0.1.2";

pub(crate) mod database;
pub mod deploy;
pub mod environment;

#[derive(Parser)]
pub enum SubCommand {
	Deploy(deploy::Opts),

	#[clap(alias = "env")]
	Environment {
		#[clap(subcommand)]
		command: environment::SubCommand,
	},

	// NOTE: This isn't an actual command, it is just added for the help entry
	/// Passthrough to the OpenGB CLI
	#[clap(name = "<any OpenGB CLI command>")]
	Any,
}

impl SubCommand {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> GlobalResult<()> {
		match self {
			SubCommand::Deploy(opts) => opts.execute(&ctx).await,
			SubCommand::Environment { command } => command.execute(&ctx).await,
			SubCommand::Any => unreachable!(),
		}
	}
}

#[derive(Debug, Deserialize)]
struct ProjectConfig {
	modules: HashMap<String, serde_json::Value>,
}

async fn read_project_config(project_path: &PathBuf) -> GlobalResult<ProjectConfig> {
	let config_path = project_path.join("backend.json");

	let project_config_str = match fs::read_to_string(&config_path).await {
		Err(err) if matches!(err.kind(), std::io::ErrorKind::NotFound) => {
			bail!("file not found: {}", config_path.display());
		}
		x => x?,
	};

	Ok(serde_json::from_str::<ProjectConfig>(&project_config_str)?)
}

/**
* Gets or auto-creates a backend project for the game.
*/
pub async fn get_or_create_project(
	ctx: &cli_core::Ctx,
) -> GlobalResult<Box<models::EeOpengbProject>> {
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

pub async fn passthrough(
	ctx: Option<&cli_core::Ctx>,
	db_command: Option<database::PassthroughSubCommand>,
) -> GlobalResult<()> {
	let mut opengb_env = vec![
		// Added to let OpenGB know its running as a passthrough command within Rivet. Shows more
		// specialized help commands.
		("RIVET_CLI_PASSTHROUGH".into(), "1".into()),
	];

	// Parse env name from: rivet backend db deploy --env foo
	if let Some(cmd) = db_command {
		let env_name_id = &cmd.get_cmd().env_name_id;
		let ctx = unwrap!(ctx, "must have ctx when running db command with --env");

		let project = get_or_create_project(ctx).await?;

		let envs_res = apis::ee_cloud_opengb_projects_envs_api::ee_cloud_opengb_projects_envs_list(
			&ctx.openapi_config_cloud,
			&project.project_id.to_string(),
			None,
		)
		.await?;

		let env = unwrap!(
			envs_res
				.environments
				.iter()
				.find(|env| &env.name_id == env_name_id),
			r#"No environment found with name id "{env_name_id}"."#,
		);

		// Read path from opengb command
		let path = if let Some(path) = cmd.path() {
			path.clone()
		} else {
			paths::project_root()?
		};

		database::provision_databases(ctx, &path, project.project_id, env.environment_id).await?;

		let databases = global_config::try_read_project(|config| {
			let project = unwrap!(config.opengb.projects.get(&project.project_id));
			let env = unwrap!(project.environments.get(&env.environment_id));

			Ok(env.databases.clone())
		})
		.await?;

		// Insert all database urls into env
		for (db_name, db) in databases {
			opengb_env.push((format!("DATABASE_URL_{}", db_name), db.url.clone()));
		}
	}

	// Match the exit code of the opengb command
	let cmd = run_opengb_command(OpenGbCommandOpts {
		args: std::env::args().skip(2).collect(),
		env: opengb_env,
		cwd: paths::project_root()?,
	})
	.await?;
	if let Some(exit_code) = cmd.code() {
		// TODO: How does this play with the sentry task?
		std::process::exit(exit_code);
	}

	Ok(())
}

pub struct OpenGbCommandOpts {
	pub args: Vec<String>,
	pub env: Vec<(String, String)>,
	pub cwd: PathBuf,
}

pub async fn run_opengb_command(opts: OpenGbCommandOpts) -> GlobalResult<std::process::ExitStatus> {
	let run_native = std::env::var("_RIVET_NATIVE_OPENGB")
		.ok()
		.map_or(false, |x| &x == "1");

	// Check OpenGB installed
	if run_native {
		ensure!(
			which::which("opengb").is_ok(),
			"OpenGB is not installed. Install it from {}.",
			term::link("https://opengb.dev/concepts/quickstart")
		);
	}

	// Build command
	if run_native {
		let mut cmd = Command::new("opengb");
		cmd.envs(opts.env);
		cmd.current_dir(opts.cwd);
		cmd.args(&opts.args);
		Ok(cmd.status().await?)
	} else {
		let image_tag = std::env::var("_RIVET_OPENGB_IMAGE")
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

		let mut cmd = Command::new("docker");
		cmd.arg("run").arg("-it");
		cmd.arg("--init");
		cmd.arg("--env-file").arg(env_file.path());
		cmd.arg("--add-host=host.docker.internal:host-gateway");
		cmd.arg("--publish=6420:6420");
		cmd.arg(format!("--volume={}:/backend", opts.cwd.display()));
		cmd.arg("--workdir=/backend");
		cmd.arg(image_tag);
		cmd.args(&opts.args);
		Ok(cmd.status().await?)
	}
}
