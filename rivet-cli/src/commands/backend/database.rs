use std::path::PathBuf;

use clap::Parser;
use toolchain_core::rivet_api::apis;
use global_error::prelude::*;
use uuid::Uuid;

use crate::util::global_config;

// Used for parsing env flag from passthrough command. Not an actual Rivet CLI command
#[derive(Parser)]
pub enum PassthroughSubCommand {
	#[clap(alias = "db")]
	Database {
		#[clap(subcommand)]
		command: DbSubCommand,

		/// The location of the OpenGB project.
		#[clap(global = true, long)]
		path: Option<PathBuf>,
	},
}

impl PassthroughSubCommand {
	pub fn get_cmd(&self) -> &Opts {
		match self {
			PassthroughSubCommand::Database { command, .. } => command.get_cmd(),
		}
	}
}

#[derive(Parser)]
pub enum DbSubCommand {
	Dev(Opts),
	Status(Opts),
	Reset(Opts),
	Deploy(Opts),
	Sh(Opts),
	Url(Opts),
}

impl DbSubCommand {
	fn get_cmd(&self) -> &Opts {
		match self {
			DbSubCommand::Dev(opts) => &opts,
			DbSubCommand::Status(opts) => &opts,
			DbSubCommand::Reset(opts) => &opts,
			DbSubCommand::Deploy(opts) => &opts,
			DbSubCommand::Sh(opts) => &opts,
			DbSubCommand::Url(opts) => &opts,
		}
	}
}

#[derive(Parser)]
pub struct Opts {
	/// The environment to deploy to.
	#[clap(long = "env")]
	pub env_name_id: String,

	/// The modules to migrate.
	#[clap(index = 1, action = clap::ArgAction::Append)]
	pub modules: Vec<String>,
}

pub async fn provision_database(
	ctx: &toolchain_core::Ctx,
	project_id: Uuid,
	env_id: Uuid,
) -> GlobalResult<()> {
	rivet_term::status::info("Provisioning databases", "");

	apis::ee_cloud_backend_projects_envs_api::ee_cloud_backend_projects_envs_provision_database(
		&ctx.openapi_config_cloud,
		&project_id.to_string(),
		&env_id.to_string(),
	)
	.await?;

	// Fetch remote DB URL
	let mut global_project_config = global_config::mutate_project(|config| {
		config
			.opengb
			.projects
			.entry(project_id)
			.or_default()
			.clone()
	})
	.await?;
	let env_config = global_project_config
		.environments
		.entry(env_id)
		.or_default();

	if env_config.url.is_none() {
		rivet_term::status::info("Fetching connection", "");

		let db_url_res =
			apis::ee_cloud_backend_projects_envs_api::ee_cloud_backend_projects_envs_get_db_url(
				&ctx.openapi_config_cloud,
				&project_id.to_string(),
				&env_id.to_string(),
			)
			.await?;

		// Add missing db url
		env_config.url = db_url_res.url;

		// Update cache
		global_config::try_mutate_project(|config| {
			// Was inserted in last `mutate_project` call
			let project = unwrap!(config.opengb.projects.get_mut(&project_id));

			project.environments.insert(env_id, env_config.clone());

			Ok(())
		})
		.await?;
	}

	Ok(())
}
