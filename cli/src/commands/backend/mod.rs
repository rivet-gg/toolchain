use std::{collections::HashMap, path::PathBuf};

use clap::Parser;
use cli_core::rivet_api::apis;
use console::Term;
use global_error::prelude::*;
use serde::Deserialize;
use tokio::fs;
use tokio::process::Command;

use crate::util::{global_config, paths, term};

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

	pub async fn passthrough(
		term: &Term,
		ctx: Option<&cli_core::Ctx>,
		db_command: Option<database::PassthroughSubCommand>,
	) -> GlobalResult<()> {
		let mut cmd = Command::new("opengb");
		let installed = cmd.output().await?.status.success();

		if !installed {
			// Prompt for OpenGB CLI install
			let install = rivet_term::prompt::PromptBuilder::default()
				.message(
					"The OpenGB CLI `opengb` is not installed. Would you like to install it now?",
				)
				.docs_url("https://github.com/rivet-gg/opengb")
				.build()?
				.bool(term)
				.await?;

			ensure!(
				install,
				"OpenGB CLI is required to use the `backend` passthrough command."
			);

			// Check if deno is installed
			let mut cmd = Command::new("deno");
			cmd.arg("--version");

			let installed = cmd.output().await?.status.success();
			ensure!(
				installed,
				"The Deno CLI tool `deno` is not installed. Install it from {}.",
				term::link("https://docs.deno.com/runtime/manual"),
			);

			// Install OpenGB CLI
			let mut cmd = Command::new("deno");
			cmd.arg("install");
			cmd.arg("--allow-net");
			cmd.arg("--allow-read");
			cmd.arg("--allow-env");
			cmd.arg("--allow-run");
			cmd.arg("--allow-write");
			cmd.arg("--name").arg("opengb");
			cmd.arg("--force");
			cmd.arg("https://raw.githubusercontent.com/rivet-gg/opengb/3aab9bc2abcb8105fc3af837900ce4f7a932ad17/src/cli/main.ts");

			ensure!(
				cmd.status().await?.success(),
				"failed to install OpenGB CLI"
			);
		}

		let mut opengb_cmd = Command::new("opengb");

		// Added to let OpenGB know its running as a passthrough command within Rivet. Shows more
		// specialized help commands.
		opengb_cmd.env("RIVET_CLI_PASSTHROUGH", "1");

		// Parse env name from: rivet backend db deploy --env foo
		if let Some(cmd) = db_command {
			let env_name_id = &cmd.get_cmd().env_name_id;
			let ctx = unwrap!(ctx, "must have ctx when running db command with --env");

			let projects_res = apis::ee_cloud_games_projects_api::ee_cloud_games_projects_list(
				&ctx.openapi_config_cloud,
				&ctx.game_id,
			)
			.await?;

			let project = unwrap!(
				projects_res.projects.first(),
				"No OpenGB projects found for the current game. Create one on the dashboard."
			);
			let project_id = project.project_id.to_string();

			let envs_res =
				apis::ee_cloud_opengb_projects_envs_api::ee_cloud_opengb_projects_envs_list(
					&ctx.openapi_config_cloud,
					&project_id,
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

			database::provision_databases(ctx, &path, project.project_id, env.environment_id)
				.await?;

			let databases = global_config::try_read_project(|config| {
				let project = unwrap!(config.opengb.projects.get(&project.project_id));
				let env = unwrap!(project.environments.get(&env.environment_id));

				Ok(env.databases.clone())
			})
			.await?;

			// Insert all database urls into env
			for (db_name, db) in databases {
				opengb_cmd.env(format!("DATABASE_URL_{}", db_name), db.url.clone());
			}
		}

		// Append arguments
		opengb_cmd.args(std::env::args().skip(2));
		opengb_cmd.current_dir(std::path::Path::new("/home/rivet/opengb/tests/basic"));

		// TODO: How does this play with the sentry task?
		// Match the exit code of the opengb command
		if let Some(exit_code) = opengb_cmd.status().await?.code() {
			std::process::exit(exit_code);
		}

		Ok(())
	}
}

#[derive(Debug, Deserialize)]
struct ProjectConfig {
	modules: HashMap<String, serde_yaml::Value>,
}

async fn read_project_config(project_path: &PathBuf) -> GlobalResult<ProjectConfig> {
	let config_path = project_path.join("backend.yaml");

	let project_config_str = match fs::read_to_string(&config_path).await {
		Err(err) if matches!(err.kind(), std::io::ErrorKind::NotFound) => {
			bail!("file not found: {}", config_path.display());
		}
		x => x?,
	};

	Ok(serde_yaml::from_str::<ProjectConfig>(&project_config_str)?)
}
