use anyhow::{Context, Result};
use clap::Parser;
use futures_util::{StreamExt, TryStreamExt};
use std::{
	env,
	sync::{
		atomic::{AtomicU64, AtomicUsize, Ordering},
		Arc,
	},
};

use crate::util::{game, upload};

#[derive(Parser)]
pub enum SubCommand {
	Push(SitePushOpts),
}

#[derive(Parser)]
pub struct SitePushOpts {
	#[clap(index(1))]
	pub path: String,

	#[clap(long)]
	pub name: Option<String>,
}

impl SubCommand {
	pub async fn execute(&self, ctx: &rivetctl::Ctx) -> Result<()> {
		match self {
			SubCommand::Push(push_opts) => {
				let reqwest_client = Arc::new(reqwest::Client::new());

				let game_id = game::infer_game_id(&ctx).await?;

				let upload_path = env::current_dir()?.join(&push_opts.path);

				let display_name = push_opts.name.clone().unwrap_or_else(|| {
					upload_path
						.file_name()
						.and_then(|n| n.to_str())
						.map(str::to_owned)
						.unwrap_or_else(|| "Site".to_owned())
				});
				println!("\n\n> Creating site \"{}\"", display_name);
				println!("  * Upload path: {}", upload_path.display());

				// Index the directory
				let files = {
					let upload_path = upload_path.clone();
					tokio::task::spawn_blocking(move || upload::prepare_upload_dir(&upload_path))
				}
				.await??;
				let total_len = files
					.iter()
					.fold(0, |acc, x| acc + x.prepared.content_length().unwrap());
				println!(
					"  * Found {count} files ({size})",
					count = files.len(),
					size = upload::format_file_size(total_len as u64)?,
				);

				// Create site
				let site_res = ctx
					.http_client
					.create_game_cdn_site()
					.game_id(&game_id)
					.display_name(&display_name)
					.set_files(Some(files.iter().map(|f| f.prepared.clone()).collect()))
					.send()
					.await
					.context("http_client.create_game_cdn_site")?;

				println!("\n\n> Uploading");
				{
					let counter = Arc::new(AtomicUsize::new(0));
					let counter_bytes = Arc::new(AtomicU64::new(0));
					let presigned_requests = site_res.presigned_requests().unwrap();
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
									let progress_bytes = counter_bytes.fetch_add(
										file.prepared.content_length().unwrap() as u64,
										Ordering::SeqCst,
									) + file.prepared.content_length().unwrap()
										as u64;
									println!(
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

				println!("\n\n> Completing");
				ctx.http_client
					.complete_upload()
					.upload_id(site_res.upload_id().unwrap())
					.send()
					.await
					.context("http_client.complete_upload")?;

				Ok(())
			}
		}
	}
}
