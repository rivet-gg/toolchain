use std::{collections::HashMap, path::PathBuf, sync::Arc};

use clap::Parser;
use cli_core::rivet_api::{apis, models};
use futures_util::{StreamExt, TryStreamExt};
use global_error::prelude::*;
use serde::Deserialize;
use tokio::fs;

use crate::{
	commands::backend::{get_or_create_project, run_opengb_command, OpenGbCommandOpts},
	util::{global_config, paths, term, upload},
};

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
		let project = get_or_create_project(ctx).await?;
		let project_id_str = project.project_id.to_string();

		let envs_res =
			apis::ee_cloud_backend_projects_envs_api::ee_cloud_backend_projects_envs_list(
				&ctx.openapi_config_cloud,
				&project_id_str,
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

		let project_path = if let Some(path) = &self.path {
			path.clone()
		} else {
			paths::project_root()?
		};

		eprintln!();
		rivet_term::status::info("Building project", project_path.display());

		// Build
		let cmd = run_opengb_command(OpenGbCommandOpts {
			args: vec![
				"build".into(),
				"--runtime".into(),
				"cloudflare-workers".into(),
			],
			env: HashMap::new(),
			cwd: project_path.clone(),
		})
		.await?;
		ensure!(cmd.success(), "Failed to build OpenGB project");

		super::database::provision_database(ctx, project.project_id, env.environment_id).await?;

		let db_url = global_config::try_read_project(|config| {
			let project = unwrap!(config.opengb.projects.get(&project.project_id));
			let env = unwrap!(project.environments.get(&env.environment_id));

			Ok(env.url.clone())
		})
		.await?;

		eprintln!();
		rivet_term::status::info("Migrating databases", "");

		// Migrate
		let mut migrate_env = HashMap::new();
		migrate_env.insert(
			"DATABASE_URL".to_string(),
			unwrap!(db_url, "no db url for env"),
		);

		let migrate_cmd = run_opengb_command(OpenGbCommandOpts {
			args: vec!["db".into(), "deploy".into()],
			env: migrate_env,
			cwd: project_path.clone(),
		})
		.await?;
		ensure!(migrate_cmd.success(), "Failed to migrate OpenGB databases");

		// Read files for upload
		let gen_manifest = read_generated_manifest(&project_path).await?;
		let bundle_path = project_path.join(gen_manifest.bundle);
		let wasm_path = gen_manifest.wasm.map(|x| project_path.join(x));
		let mut files = vec![upload::prepare_upload_file(
			&bundle_path,
			"bundle.js",
			fs::metadata(&bundle_path).await?,
		)?];
		if let Some(wasm) = wasm_path.as_ref() {
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
			apis::ee_cloud_backend_projects_envs_api::ee_cloud_backend_projects_envs_prepare_deploy(
				&ctx.openapi_config_cloud,
				&project_id_str,
				&env_id,
				models::EeCloudBackendProjectsEnvsPrepareDeployRequest {
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

		eprintln!();
		rivet_term::status::info("Deploying environment", &env.display_name);

		let deploy_res =
			apis::ee_cloud_backend_projects_envs_api::ee_cloud_backend_projects_envs_deploy(
				&ctx.openapi_config_cloud,
				&project_id_str,
				&env_id,
				models::EeCloudBackendProjectsEnvsDeployRequest {
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

async fn read_generated_manifest(project_path: &PathBuf) -> GlobalResult<GenManifest> {
	let manifest_str =
		fs::read_to_string(project_path.join(".opengb").join("manifest.json")).await?;
	Ok(serde_json::from_str::<GenManifest>(&manifest_str)?)
}
