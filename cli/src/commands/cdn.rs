use clap::Parser;
use cli_core::rivet_api::{apis, models};
use futures_util::{StreamExt, TryStreamExt};
use global_error::prelude::*;
use serde::Serialize;
use std::{
	env,
	sync::{
		atomic::{AtomicU64, AtomicUsize, Ordering},
		Arc,
	},
};
use tokio::process::Command;
use uuid::Uuid;

use crate::util::{struct_fmt, term, upload};

#[derive(Parser)]
pub enum SubCommand {
	Push(PushOpts),
	BuildPush(BuildPushOpts),
}

#[derive(Parser)]
pub struct PushOpts {
	/// Path of the site to push
	#[clap(long)]
	pub path: String,

	/// Name of the build
	#[clap(long)]
	pub name: Option<String>,

	/// Number of files to upload in parallel
	#[clap(long, env = "RIVET_CONCURRENT_UPLOADS", default_value = "8")]
	pub concurrent_uploads: usize,

	#[clap(long, value_parser)]
	pub format: Option<struct_fmt::Format>,
}

impl SubCommand {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> GlobalResult<()> {
		match self {
			SubCommand::Push(push_opts) => {
				let output = push(ctx, push_opts).await?;
				struct_fmt::print_opt(push_opts.format.as_ref(), &output)?;
				Ok(())
			}
			SubCommand::BuildPush(push_opts) => {
				let output = build_and_push(ctx, push_opts).await?;
				struct_fmt::print_opt(push_opts.format.as_ref(), &output)?;
				Ok(())
			}
		}
	}
}

#[derive(Serialize)]
pub struct PushOutput {
	pub site_id: Uuid,
}

pub async fn push(ctx: &cli_core::Ctx, push_opts: &PushOpts) -> GlobalResult<PushOutput> {
	let reqwest_client = Arc::new(reqwest::Client::new());

	let upload_path = env::current_dir()?.join(&push_opts.path);

	let display_name = push_opts.name.clone().unwrap_or_else(|| {
		upload_path
			.file_name()
			.and_then(|n| n.to_str())
			.map(str::to_owned)
			.unwrap_or_else(|| "Site".to_owned())
	});

	// Index the directory
	let files = {
		let upload_path = upload_path.clone();
		tokio::task::spawn_blocking(move || upload::prepare_upload_dir(&upload_path))
	}
	.await??;
	let total_len = files
		.iter()
		.fold(0, |acc, x| acc + x.prepared.content_length);

	eprintln!();
	term::status::info(
		"Uploading Site",
		format!(
			"{name} ({count} files, {size} total)",
			name = display_name,
			count = files.len(),
			size = upload::format_file_size(total_len as u64)?,
		),
	);

	eprintln!("  * Upload path: {}", upload_path.display());
	// Create site
	let site_res = apis::cloud_games_cdn_api::cloud_games_cdn_create_game_cdn_site(
		&ctx.openapi_config_cloud,
		&ctx.game_id,
		models::CloudGamesCreateGameCdnSiteRequest {
			display_name: display_name.clone(),
			files: files.iter().map(|f| f.prepared.clone()).collect(),
		},
	)
	.await;
	if let Err(err) = site_res.as_ref() {
		println!("Error: {err:?}");
	}
	let site_res = unwrap!(site_res);
	let site_id = site_res.site_id;

	{
		let counter = Arc::new(AtomicUsize::new(0));
		let counter_bytes = Arc::new(AtomicU64::new(0));
		let presigned_requests = site_res.presigned_requests;
		let total = presigned_requests.len();
		let total_bytes = total_len as u64;

		let files = Arc::new(files.clone());
		futures_util::stream::iter(presigned_requests)
			.map(Ok)
			.try_for_each_concurrent(push_opts.concurrent_uploads, move |presigned_req| {
				let counter = counter.clone();
				let counter_bytes = counter_bytes.clone();
				{
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
						)
						.await?;

						let progress = counter.fetch_add(1, Ordering::SeqCst) + 1;
						let progress_bytes = counter_bytes
							.fetch_add(file.prepared.content_length as u64, Ordering::SeqCst)
							+ file.prepared.content_length as u64;
						eprintln!(
							"    {}/{} files ({}/{})",
							progress,
							total,
							upload::format_file_size(progress_bytes)?,
							upload::format_file_size(total_bytes)?
						);

						GlobalResult::<()>::Ok(())
					}
				}
			})
			.await?;
	}

	eprintln!();
	let complete_res = apis::cloud_uploads_api::cloud_uploads_complete_upload(
		&ctx.openapi_config_cloud,
		&site_res.upload_id.to_string(),
	)
	.await;
	if let Err(err) = complete_res.as_ref() {
		println!("Error: {err:?}");
	}
	unwrap!(complete_res);
	term::status::success("Site Upload Complete", site_id);

	Ok(PushOutput { site_id })
}

#[derive(Parser)]
pub struct BuildPushOpts {
	/// Command to run before pushing
	///
	/// The `RIVET_API_ENDPOINT` environment variable will be exposed to this command. The
	/// `RIVET_TOKEN` environment variable will be removed for security.
	#[clap(long)]
	pub command: String,

	/// Path of the site to push
	#[clap(long)]
	pub path: String,

	/// Name of the build
	#[clap(long)]
	pub name: Option<String>,

	/// Number of files to upload in parallel
	#[clap(long, env = "RIVET_CONCURRENT_UPLOADS", default_value = "8")]
	pub concurrent_uploads: usize,

	#[clap(long, value_parser)]
	pub format: Option<struct_fmt::Format>,
}

pub async fn build_and_push(
	ctx: &cli_core::Ctx,
	push_opts: &BuildPushOpts,
) -> GlobalResult<PushOutput> {
	eprintln!();
	term::status::info("Building Site", &push_opts.command);

	if cfg!(unix) {
		let mut build_cmd = Command::new("/bin/sh");
		build_cmd
			.env("RIVET_API_ENDPOINT", &ctx.api_endpoint)
			// Ensure we don't accidentally expose the token to a public build
			.env_remove("RIVET_TOKEN")
			.arg("-c")
			.arg(&push_opts.command);
		let build_status = build_cmd.status().await?;
		ensure!(build_status.success(), "site failed to build");
	} else if cfg!(windows) {
		let mut build_cmd = Command::new("cmd.exe");
		build_cmd
			.env("RIVET_API_ENDPOINT", &ctx.api_endpoint)
			// Ensure we don't accidentally expose the token to a public build
			.env_remove("RIVET_TOKEN")
			.arg("/C")
			.arg(&push_opts.command);
		let build_status = build_cmd.status().await?;
		ensure!(build_status.success(), "site failed to build");
	} else {
		bail!("unknown machine type, expected unix or windows")
	};

	// Upload site
	push(
		ctx,
		&PushOpts {
			path: push_opts.path.clone(),
			name: push_opts.name.clone(),
			concurrent_uploads: push_opts.concurrent_uploads,
			format: push_opts.format.clone(),
		},
	)
	.await
}
