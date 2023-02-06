use anyhow::{Context, Result};
use clap::Parser;
use cli_core::rivet_api;
use futures_util::{StreamExt, TryStreamExt};
use serde::Serialize;
use std::{
	env,
	sync::{
		atomic::{AtomicU64, AtomicUsize, Ordering},
		Arc,
	},
};

use crate::util::{struct_fmt, upload};

#[derive(Parser)]
pub enum SubCommand {
	Push(SitePushOpts),
}

#[derive(Parser)]
pub struct SitePushOpts {
	/// Path of the site to push
	#[clap(long)]
	pub path: String,

	/// Name of the build
	#[clap(long)]
	pub name: Option<String>,

	#[clap(long, value_parser)]
	pub format: Option<struct_fmt::Format>,
}

impl SubCommand {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> Result<()> {
		match self {
			SubCommand::Push(push_opts) => {
				let output = push(ctx, push_opts).await?;
				struct_fmt::print_opt(push_opts.format.as_ref(), &output)?;
				Ok(())
			}
		}
	}
}

#[derive(Serialize)]
pub struct PushOutput {
	pub site_id: String,
}

pub async fn push(ctx: &cli_core::Ctx, push_opts: &SitePushOpts) -> Result<PushOutput> {
	let reqwest_client = Arc::new(reqwest::Client::new());

	let upload_path = env::current_dir()?.join(&push_opts.path);

	let display_name = push_opts.name.clone().unwrap_or_else(|| {
		upload_path
			.file_name()
			.and_then(|n| n.to_str())
			.map(str::to_owned)
			.unwrap_or_else(|| "Site".to_owned())
	});
	eprintln!("\n\n> Pushing site \"{}\"", display_name);
	eprintln!("  * Upload path: {}", upload_path.display());

	// Index the directory
	let files = {
		let upload_path = upload_path.clone();
		tokio::task::spawn_blocking(move || upload::prepare_upload_dir(&upload_path))
	}
	.await??;
	let total_len = files
		.iter()
		.fold(0, |acc, x| acc + x.prepared.content_length);
	eprintln!(
		"  * Found {count} files ({size})",
		count = files.len(),
		size = upload::format_file_size(total_len as u64)?,
	);

	// Create site
	let site_res = rivet_api::apis::cloud_games_cdn_api::cloud_games_cdn_create_game_cdn_site(
		&ctx.openapi_config_cloud,
		&ctx.game_id,
		rivet_api::models::CloudGamesCreateGameCdnSiteInput {
			display_name: display_name.clone(),
			files: files.iter().map(|f| f.prepared.clone()).collect(),
		},
	)
	.await;
	if let Err(err) = site_res.as_ref() {
		println!("Error: {err:?}");
	}
	let site_res = site_res.context("cloud_games_cdn_create_game_cdn_site")?;
	let site_id = site_res.site_id;

	eprintln!("\n\n> Uploading");
	{
		let counter = Arc::new(AtomicUsize::new(0));
		let counter_bytes = Arc::new(AtomicU64::new(0));
		let presigned_requests = site_res.presigned_requests;
		let total = presigned_requests.len();
		let total_bytes = total_len as u64;

		let files = Arc::new(files.clone());
		futures_util::stream::iter(presigned_requests)
			.map(Ok)
			.try_for_each_concurrent(ctx.concurrent_uploads, move |presigned_req| {
				let counter = counter.clone();
				let counter_bytes = counter_bytes.clone();
				{
					let files = files.clone();
					let reqwest_client = reqwest_client.clone();

					async move {
						// Find the matching prepared file
						let file = files
							.iter()
							.find(|f| f.prepared.path == presigned_req.path)
							.context("missing prepared file")?;

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

						Result::<()>::Ok(())
					}
				}
			})
			.await?;
	}

	eprintln!("\n\n> Completing");
	let complete_res = rivet_api::apis::cloud_uploads_api::cloud_uploads_complete_upload(
		&ctx.openapi_config_cloud,
		&site_res.upload_id,
	)
	.await;
	if let Err(err) = complete_res.as_ref() {
		println!("Error: {err:?}");
	}
	complete_res.context("cloud_uploads_complete_upload")?;

	Ok(PushOutput {
		site_id: site_id.to_string(),
	})
}
