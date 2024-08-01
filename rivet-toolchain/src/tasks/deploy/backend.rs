use futures_util::{StreamExt, TryStreamExt};
use global_error::prelude::*;
use rivet_api::{apis, models};
use serde::Deserialize;
use std::{
	collections::HashMap,
	path::{Path, PathBuf},
	sync::Arc,
};
use tokio::fs;

use crate::{backend, config, ctx::Ctx, util::net::upload, util::task::TaskCtx};

const OPENGB_NO_MINIFY: bool = false;

pub struct DeployOpts {
	/// The environment to deploy to.
	pub environment_id: String,

	/// The location of the OpenGB project.
	pub project_path: String,

	/// Skip the migration step.
	pub skip_migrate: bool,
}

pub async fn deploy(ctx: &Ctx, task: TaskCtx, opts: DeployOpts) -> GlobalResult<()> {
	task.log_stdout("[Deploying Backend]");

	let project = backend::get_or_create_project(ctx).await?;
	let project_id_str = project.project_id.to_string();
	let environment_id_str = opts.environment_id.to_string();
	let project_path = PathBuf::from(opts.project_path.clone());

	let env = apis::ee_cloud_backend_projects_envs_api::ee_cloud_backend_projects_envs_get(
		&ctx.openapi_config_cloud,
		&project_id_str,
		&environment_id_str,
		None,
	)
	.await?
	.environment;

	task.log_stdout(format!("[Building Project] {}", project_path.display()));

	// Build
	let mut cmd_env = HashMap::new();
	if OPENGB_NO_MINIFY {
		cmd_env.insert("_OPENGB_ESBUILD_NO_MINIFY".into(), "1".into());
	}
	let cmd = backend::run_opengb_command(
		task.clone(),
		backend::OpenGbCommandOpts {
			opengb_target: backend::OpenGbTarget::Native,
			args: vec![
				"build".into(),
				"--db-driver".into(),
				"neon-serverless".into(),
				"--runtime".into(),
				"cloudflare-workers-platforms".into(),
			],
			env: cmd_env,
			cwd: project_path.clone(),
		},
	)
	.await?;
	ensure!(cmd == 0, "Failed to build OpenGB project");

	backend::database::provision_database(
		task.clone(),
		ctx,
		project.project_id,
		env.environment_id,
	)
	.await?;

	let db_url = config::global::try_read_project(|config| {
		let project = unwrap!(config.opengb.projects.get(&project.project_id));
		let env = unwrap!(project.environments.get(&env.environment_id));

		Ok(env.url.clone())
	})
	.await?;

	if !opts.skip_migrate {
		task.log_stdout("[Migrating Database]");

		// Migrate
		let mut migrate_env = HashMap::new();
		migrate_env.insert(
			"DATABASE_URL".to_string(),
			unwrap!(db_url, "no db url for env"),
		);

		let migrate_cmd = backend::run_opengb_command(
			task.clone(),
			backend::OpenGbCommandOpts {
				opengb_target: backend::OpenGbTarget::Native,
				args: vec!["db".into(), "deploy".into()],
				env: migrate_env,
				cwd: project_path.clone(),
			},
		)
		.await?;
		ensure!(migrate_cmd == 0, "Failed to migrate OpenGB databases");
	}

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

	task.log_stdout(format!(
		"[Uploading Environment] {name} ({count} files, {size} total)",
		name = &env.display_name,
		count = files.len(),
		size = upload::format_file_size(total_len as u64)?,
	));

	let prepare_res = unwrap!(
		apis::ee_cloud_backend_projects_envs_api::ee_cloud_backend_projects_envs_prepare_deploy(
			&ctx.openapi_config_cloud,
			&project_id_str,
			&environment_id_str,
			models::EeCloudBackendProjectsEnvsPrepareDeployRequest {
				files: files.iter().map(|f| f.prepared.clone()).collect(),
			},
		)
		.await
	);

	// Upload files
	let reqwest_client = Arc::new(reqwest::Client::new());
	// let pb = term::EitherProgressBar::Multi(indicatif::MultiProgress::new());

	futures_util::stream::iter(prepare_res.presigned_requests)
		.map(Ok)
		.try_for_each_concurrent(8, |presigned_req| {
			// let pb = pb.clone();
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
					// pb,
				)
				.await?;

				GlobalResult::<()>::Ok(())
			}
		})
		.await?;

	task.log_stdout(format!("[Deploying Environment] {}", env.display_name));

	let deploy_res =
		apis::ee_cloud_backend_projects_envs_api::ee_cloud_backend_projects_envs_deploy(
			&ctx.openapi_config_cloud,
			&project_id_str,
			&environment_id_str,
			models::EeCloudBackendProjectsEnvsDeployRequest {
				upload_id: prepare_res.upload_id,
			},
		)
		.await?;

	task.log_stdout(format!("[Done] OpenGB API available at {}", deploy_res.url));

	Ok(())
}

#[derive(Deserialize)]
struct GenManifest {
	bundle: String,
	wasm: Option<String>,
}

async fn read_generated_manifest(project_path: &Path) -> GlobalResult<GenManifest> {
	let manifest_str =
		fs::read_to_string(project_path.join(".opengb").join("manifest.json")).await?;
	Ok(serde_json::from_str::<GenManifest>(&manifest_str)?)
}
