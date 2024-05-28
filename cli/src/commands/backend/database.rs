use std::{collections::HashMap, path::PathBuf};

use clap::Parser;
use cli_core::rivet_api::{apis, models};
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
	pub fn path(&self) -> Option<&PathBuf> {
		match self {
			PassthroughSubCommand::Database { path, .. } => path.as_ref(),
		}
	}

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

pub async fn provision_databases(
	ctx: &cli_core::Ctx,
	opengb_project_path: &PathBuf,
	project_id: Uuid,
	env_id: Uuid,
) -> GlobalResult<()> {
	rivet_term::status::info("Provisioning databases", "");

	// Structure DB names
	let project_config = super::read_project_config(opengb_project_path).await?;
	let modules = project_config
		.modules
		.keys()
		.map(|module_name| {
			(
				module_name.clone(),
				models::EeOpengbModule {
					db: Some(Box::new(models::EeOpengbModuleDb {
						name: format!("module_{module_name}"),
					})),
				},
			)
		})
		.collect::<HashMap<_, _>>();

	apis::ee_cloud_opengb_projects_envs_api::ee_cloud_opengb_projects_envs_provision_databases(
		&ctx.openapi_config_cloud,
		&project_id.to_string(),
		&env_id.to_string(),
		models::EeCloudOpengbProjectsEnvsProvisionDatabasesRequest { modules },
	)
	.await?;

	// Fetch remote DB URLs
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

	let missing_dbs = project_config
		.modules
		.keys()
		.map(|module_name| format!("module_{module_name}"))
		.filter(|db_name| env_config.databases.get(db_name).is_none())
		.collect::<Vec<_>>();

	if !missing_dbs.is_empty() {
		rivet_term::status::info("Fetching connections", "");

		let db_urls_res = ee_cloud_opengb_projects_envs_get_db_urls(
			&ctx.openapi_config_cloud,
			&project_id.to_string(),
			&env_id.to_string(),
			missing_dbs,
		)
		.await?;

		// Add missing db urls
		env_config.databases.extend(
			db_urls_res
				.databases
				.into_iter()
				.map(|(db_name, db_url)| (db_name, global_config::OpenGbDatabase { url: db_url })),
		);

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

// The OpenAPI generated client doesn't handle list query parameters correctly, have to patch it here
pub async fn ee_cloud_opengb_projects_envs_get_db_urls(
	configuration: &apis::configuration::Configuration,
	project_id: &str,
	environment_id: &str,
	dbs: Vec<String>,
) -> Result<
	models::EeCloudOpengbProjectsEnvsGetDbUrlsResponse,
	apis::Error<apis::ee_cloud_opengb_projects_envs_api::EeCloudOpengbProjectsEnvsGetDbUrlsError>,
> {
	let local_var_configuration = configuration;

	let local_var_client = &local_var_configuration.client;

	let local_var_uri_str = format!(
		"{}/cloud/opengb/projects/{project_id}/environments/{environment_id}/db",
		local_var_configuration.base_path,
		project_id = apis::urlencode(project_id),
		environment_id = apis::urlencode(environment_id)
	);
	let mut local_var_req_builder =
		local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

	local_var_req_builder = match "multi" {
		"multi" => local_var_req_builder.query(
			&dbs.into_iter()
				.map(|p| ("dbs".to_owned(), p))
				.collect::<Vec<_>>(),
		),
		_ => local_var_req_builder.query(&[(
			"dbs",
			&dbs.into_iter()
				.map(|p| p.to_string())
				.collect::<Vec<_>>()
				.join(",")
				.to_string(),
		)]),
	};
	if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
		local_var_req_builder =
			local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
	}
	if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
		local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
	};

	let local_var_req = local_var_req_builder.build()?;
	let local_var_resp = local_var_client.execute(local_var_req).await?;

	let local_var_status = local_var_resp.status();
	let local_var_content = local_var_resp.text().await?;

	if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
		serde_json::from_str(&local_var_content).map_err(apis::Error::from)
	} else {
		let local_var_entity: Option<
			apis::ee_cloud_opengb_projects_envs_api::EeCloudOpengbProjectsEnvsGetDbUrlsError,
		> = serde_json::from_str(&local_var_content).ok();
		let local_var_error = apis::ResponseContent {
			status: local_var_status,
			content: local_var_content,
			entity: local_var_entity,
		};
		Err(apis::Error::ResponseError(local_var_error))
	}
}
