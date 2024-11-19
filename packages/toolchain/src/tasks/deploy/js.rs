use anyhow::*;
use futures_util::{StreamExt, TryStreamExt};
use rivet_api::{apis, models};
use std::{path::PathBuf, sync::Arc};
use tokio::fs;
use uuid::Uuid;

use crate::{
	config, paths,
	project::environment::TEMPEnvironment,
	toolchain_ctx::ToolchainCtx,
	util::{net::upload, task, term},
};

const BUILD_INDEX_NAME: &str = "index.js";

pub struct BuildAndUploadOpts {
	pub env: TEMPEnvironment,
	pub config: config::Config,
	pub build_config: config::build::javascript::Build,
	pub version_name: String,
}

/// Builds image if not specified and returns the build ID.
pub async fn build_and_upload(
	ctx: &ToolchainCtx,
	task: task::TaskCtx,
	opts: BuildAndUploadOpts,
) -> Result<Uuid> {
	task.log("[Build]");

	let project_root = paths::project_root()?;

	// Determine build attributes
	let build_config_unstable = opts.build_config.unstable();
	let compression = build_config_unstable.compression();

	let build_dir = tempfile::TempDir::new()?;

	match opts.build_config.bundler {
		config::build::javascript::Bundler::None(opts) => {
			// Copy index file to build dir
			fs::copy(
				project_root.join(opts.index_path),
				build_dir.path().join(BUILD_INDEX_NAME),
			)
			.await?;
		}
	};

	// Deploy JS build
	let build_id = upload_bundle(
		ctx,
		task.clone(),
		&UploadBundleOpts {
			config: opts.config.clone(),
			env: opts.env,
			version_name: opts.version_name,
			build_path: build_dir.path().into(),
			build_manifest: BuildManifest {
				files: vec![BUILD_INDEX_NAME.into()],
			},
			compression,
		},
	)
	.await?;

	Ok(build_id)
}

// pub struct JsBundleOpts {
// 	pub config: config::Config,
// 	pub env: TEMPEnvironment,
// }
//
// pub struct JsBundleOutput {
// 	pub path: PathBuf,
// 	pub manifest: BuildManafest,
// }
//
// /// Bundle a JS build
// pub async fn bundle_js(
// 	ctx: &ToolchainCtx,
// 	task: task::TaskCtx,
// 	current_dir: &Path,
// 	bundle_opts: &JsBundleOpts,
// ) -> Result<()> {
// 	todo!()
// }

struct UploadBundleOpts {
	config: config::Config,
	env: TEMPEnvironment,
	version_name: String,

	/// Path to the root of the built files.
	build_path: PathBuf,

	/// Manifest of files in the bundle.
	build_manifest: BuildManifest,

	compression: config::build::Compression,
}

/// Uploads a built JavaScript bundle.
async fn upload_bundle(
	ctx: &ToolchainCtx,
	task: task::TaskCtx,
	push_opts: &UploadBundleOpts,
) -> Result<Uuid> {
	let multipart_enabled: bool = push_opts.config.unstable().multipart_enabled();

	// Prepare index for upload
	ensure!(
		push_opts.build_manifest.files.len() == 1,
		"must only upload bundle file"
	);
	ensure!(
		push_opts.build_manifest.files[0] == BUILD_INDEX_NAME,
		"build file must be named `{}`",
		BUILD_INDEX_NAME
	);
	let index_path = push_opts
		.build_path
		.join(&push_opts.build_manifest.files[0]);
	let image_file = upload::prepare_upload_file(
		&index_path,
		&push_opts.build_manifest.files[0],
		fs::metadata(&index_path).await?,
	)?;
	let files = vec![image_file.clone()];

	let total_len = files
		.iter()
		.fold(0, |acc, x| acc + x.prepared.content_length);

	// task.log(format!(
	// 	"[Uploading Build] {count} files, {size} total",
	// 	count = files.len(),
	// 	size = upload::format_file_size(total_len as u64)?,
	// ));
	task.log(format!(
		"[Uploading Build] {size}",
		size = upload::format_file_size(total_len as u64)?,
	));

	let prepare_res = apis::actor_builds_api::actor_builds_prepare(
		&ctx.openapi_config_cloud,
		models::ActorPrepareBuildRequest {
			name: push_opts.version_name.clone(),
			image_tag: None,
			image_file: Box::new(image_file.prepared),
			kind: Some(models::ActorBuildKind::Javascript),
			compression: Some(match push_opts.compression {
				config::build::Compression::None => models::ActorBuildCompression::None,
				config::build::Compression::Lz4 => models::ActorBuildCompression::Lz4,
			}),
			multipart_upload: Some(multipart_enabled),
			// TODO(RVT-4124):
			prewarm_regions: None,
		},
		Some(&ctx.project.name_id),
		Some(&push_opts.env.slug),
	)
	.await
	.map_err(|err| anyhow!("Failed to prepare deploy: {err}"))?;

	// Upload files
	let reqwest_client = Arc::new(reqwest::Client::new());
	let pb = term::EitherProgressBar::Multi(term::multi_progress_bar(task.clone()));

	let presigned_requests = if let Some(presigned_requests) = prepare_res.image_presigned_requests
	{
		presigned_requests
	} else if let Some(image_presigned_request) = prepare_res.image_presigned_request {
		vec![*image_presigned_request]
	} else {
		bail!("neither `image_presigned_request` or `image_presigned_requests` provided")
	};
	futures_util::stream::iter(presigned_requests)
		.map(Ok)
		.try_for_each_concurrent(8, |presigned_req| {
			let task = task.clone();
			let pb = pb.clone();
			let files = files.clone();
			let reqwest_client = reqwest_client.clone();

			async move {
				// Find the matching prepared file
				let file = files
					.iter()
					.find(|f| f.prepared.path == presigned_req.path)
					.context("missing prepared file")?;

				upload::upload_file(
					task.clone(),
					&reqwest_client,
					&presigned_req,
					&file.absolute_path,
					file.prepared.content_type.as_ref(),
					pb,
				)
				.await?;

				Result::<()>::Ok(())
			}
		})
		.await?;

	let complete_res = apis::actor_builds_api::actor_builds_complete(
		&ctx.openapi_config_cloud,
		&prepare_res.build.to_string(),
		Some(&ctx.project.name_id),
		Some(&push_opts.env.slug),
	)
	.await;
	if let Err(err) = complete_res.as_ref() {
		task.log(format!("{err:?}"));
	}
	complete_res.context("complete_res")?;

	Ok(prepare_res.build)
}

struct BuildManifest {
	files: Vec<String>,
}
