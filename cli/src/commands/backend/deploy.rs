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

		super::database::provision_databases(ctx, &path, project.project_id, env.environment_id)
			.await?;

		let databases = global_config::try_read_project(|config| {
			let project = unwrap!(config.opengb.projects.get(&project.project_id));
			let env = unwrap!(project.environments.get(&env.environment_id));

			Ok(env.databases.clone())
		})
		.await?;

		rivet_term::status::info("Migrating databases", "");

		let mut migrate_cmd = Command::new("opengb");
		migrate_cmd.current_dir(&path);
		migrate_cmd.arg("db").arg("deploy");

		// Insert all database urls into env
		for (db_name, db) in databases {
			migrate_cmd.env(format!("DATABASE_URL_{}", db_name), db.url.clone());
		}

		ensure!(
			migrate_cmd.status().await?.success(),
			"Failed to migrate OpenGB databases",
		);

		// Read files for upload
		let gen_manifest = read_gen_manifest(&path).await?;
		let mut files = vec![upload::prepare_upload_file(
			&gen_manifest.bundle,
			"bundle.js",
			fs::metadata(&gen_manifest.bundle).await?,
		)?];
		if let Some(wasm) = gen_manifest.wasm.as_ref() {
			files.push(upload::prepare_upload_file(
				wasm,
				"query-engine.wasm",
				fs::metadata(wasm).await?,
			)?);
		}
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

		// Structure DB names
		let project_config = super::read_project_config(&path).await?;
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

#[derive(Deserialize)]
struct GenManifest {
	bundle: String,
	wasm: Option<String>,
}

async fn read_gen_manifest(project_path: &PathBuf) -> GlobalResult<GenManifest> {
	let manifest_str = fs::read_to_string(project_path.join("_gen").join("manifest.json")).await?;
	Ok(serde_json::from_str::<GenManifest>(&manifest_str)?)
}
