use std::{collections::HashMap, path::PathBuf, sync::Arc};

use clap::Parser;
use cli_core::rivet_api::{apis, models};
use futures_util::{StreamExt, TryStreamExt};
use global_error::prelude::*;
use serde::Deserialize;
use tokio::{fs, process::Command};

use crate::util::{global_config, paths, term, upload};

#[derive(Parser)]
pub struct Opts {
	/// The environment to deploy to.
	#[clap(index = 1)]
	env_name_id: String,

	/// The location of the OpenGB project.
	#[clap(long)]
	path: Option<std::path::PathBuf>,
}

impl Opts {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> GlobalResult<()> {
		let projects_res = apis::ee_cloud_games_projects_api::ee_cloud_games_projects_list(
			&ctx.openapi_config_cloud,
			&ctx.game_id,
		)
		.await?;

		// TODO: Add a link to the dashboard in this error message
		let project = unwrap!(
			projects_res.projects.first(),
			"No OpenGB projects found for the current game. Create one on the dashboard."
		);
		let project_id = project.project_id.to_string();

		let envs_res = apis::ee_cloud_opengb_projects_envs_api::ee_cloud_opengb_projects_envs_list(
			&ctx.openapi_config_cloud,
			&project_id,
			None,
		)
		.await?;

		let env = unwrap!(
			envs_res
				.environments
				.iter()
				.find(|env| env.name_id == self.env_name_id),
			r#"No environment found with name id "{}"."#,
			self.env_name_id,
		);
		let env_id = env.environment_id.to_string();

		let path = if let Some(path) = &self.path {
			path.clone()
		} else {
			paths::project_root()?
		};

		rivet_term::status::info("Building project", path.display());

		let mut build_cmd = Command::new("opengb");
		build_cmd.current_dir(&path);
		build_cmd.arg("build");
		build_cmd.arg("--runtime").arg("cf");

		ensure!(
			build_cmd.status().await?.success(),
			"Failed to build OpenGB project"
		);

		rivet_term::status::info("Provisioning databases", "");

		// Structure DB names
		let project_config = read_project_config(&path).await?;
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
			&project_id,
			&env_id,
			models::EeCloudOpengbProjectsEnvsProvisionDatabasesRequest {
				modules: modules.clone(),
			},
		)
		.await?;

		rivet_term::status::info("Fetching connections", "");

		// Fetch remote DB URLs
		let mut global_project_config = global_config::mutate_project(|config| {
			config
				.opengb
				.projects
				.entry(project.project_id)
				.or_default()
				.clone()
		})
		.await?;
		let env_config = global_project_config
			.environments
			.entry(env.environment_id)
			.or_default();

		let missing_dbs = project_config
			.modules
			.keys()
			.map(|module_name| format!("module_{module_name}"))
			.filter(|db_name| env_config.databases.get(db_name).is_none())
			.collect::<Vec<_>>();

		if !missing_dbs.is_empty() {
			let db_urls_res = ee_cloud_opengb_projects_envs_get_db_urls(
				&ctx.openapi_config_cloud,
				&project_id,
				&env_id,
				missing_dbs,
			)
			.await?;

			// Add missing db urls
			env_config.databases.extend(
				db_urls_res.databases.into_iter().map(|(db_name, db_url)| {
					(db_name, global_config::OpenGbDatabase { url: db_url })
				}),
			);

			// Update cache
			global_config::mutate_project(|config| {
				config
					.opengb
					.projects
					.get_mut(&project.project_id)
					// Was inserted in last `mutate_project` call
					.unwrap()
					.environments
					.insert(env.environment_id, env_config.clone());
			})
			.await?;
		}

		rivet_term::status::info("Migrating databases", "");

		let mut migrate_cmd = Command::new("opengb");
		migrate_cmd.current_dir(&path);
		migrate_cmd.arg("db").arg("deploy");

		// Insert all database urls into env
		for (db_name, db) in &env_config.databases {
			migrate_cmd.env(format!("DATABASE_URL_{}", db_name), db.url.clone());
		}

		ensure!(
			migrate_cmd.status().await?.success(),
			"Failed to migrate OpenGB databases"
		);

		// Read files for upload
		let gen_manifest = read_gen_manifest(&path).await?;
		let files = vec![
			upload::prepare_upload_file(
				&gen_manifest.bundle,
				"bundle.js",
				fs::metadata(&gen_manifest.bundle).await?,
			)?,
			// TODO: Get rid of unwrap
			upload::prepare_upload_file(
				gen_manifest.wasm.as_ref().unwrap(),
				"query-engine.wasm",
				fs::metadata(gen_manifest.wasm.as_ref().unwrap()).await?,
			)?,
		];
		let total_len = files
			.iter()
			.fold(0, |acc, x| acc + x.prepared.content_length);

		eprintln!();
		rivet_term::status::info(
			"Uploading Environment",
			format!(
				"{name} ({count} files, {size} total)",
				name = &env.display_name,
				count = files.len(),
				size = upload::format_file_size(total_len as u64)?,
			),
		);

		let prepare_res = unwrap!(
			apis::ee_cloud_opengb_projects_envs_api::ee_cloud_opengb_projects_envs_prepare_deploy(
				&ctx.openapi_config_cloud,
				&project_id,
				&env_id,
				models::EeCloudOpengbProjectsEnvsPrepareDeployRequest {
					files: files.iter().map(|f| f.prepared.clone()).collect(),
				},
			)
			.await
		);

		// Upload files
		let reqwest_client = Arc::new(reqwest::Client::new());
		let pb = term::EitherProgressBar::Multi(indicatif::MultiProgress::new());

		futures_util::stream::iter(prepare_res.presigned_requests)
			.map(Ok)
			.try_for_each_concurrent(8, |presigned_req| {
				let pb = pb.clone();
				let files = files.clone();
				let reqwest_client = reqwest_client.clone();

				async move {
					// Find the matching prepared file
					let file = unwrap!(
						files.iter().find(|f| f.prepared.path == presigned_req.path),
						"missing prepared file"
					);

					upload::upload_file(
						&reqwest_client,
						&presigned_req,
						&file.absolute_path,
						file.prepared.content_type.as_ref(),
						pb,
					)
					.await?;

					GlobalResult::<()>::Ok(())
				}
			})
			.await?;

		eprintln!("\n");
		rivet_term::status::info("Deploying environment", &env.display_name);

		let deploy_res =
			apis::ee_cloud_opengb_projects_envs_api::ee_cloud_opengb_projects_envs_deploy(
				&ctx.openapi_config_cloud,
				&project_id,
				&env_id,
				models::EeCloudOpengbProjectsEnvsDeployRequest {
					modules,
					upload_id: prepare_res.upload_id,
				},
			)
			.await?;

		eprintln!("");
		rivet_term::status::success(
			"Done",
			format!("OpenGB API available at {}", term::link(deploy_res.url)),
		);

		Ok(())
	}
}

#[derive(Debug, Deserialize)]
struct ProjectConfig {
	modules: HashMap<String, serde_yaml::Value>,
}

async fn read_project_config(project_path: &PathBuf) -> GlobalResult<ProjectConfig> {
	let project_config_str = fs::read_to_string(project_path.join("backend.yaml")).await?;
	Ok(serde_yaml::from_str::<ProjectConfig>(&project_config_str)?)
}

#[derive(Deserialize)]
struct GenManifest {
	bundle: String,
	wasm: Option<String>,
}

async fn read_gen_manifest(project_path: &PathBuf) -> GlobalResult<GenManifest> {
	let manifest_str = fs::read_to_string(project_path.join("_gen").join("manifest.json")).await?;
	Ok(serde_json::from_str::<GenManifest>(&manifest_str)?)
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
