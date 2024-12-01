use anyhow::*;
use base64::{engine::general_purpose::STANDARD, Engine};
use clap::ValueEnum;
use toolchain::rivet_api::{apis, models};
use uuid::Uuid;

#[derive(ValueEnum, Clone)]
pub enum LogStream {
	#[clap(name = "stdout")]
	StdOut,
	#[clap(name = "stderr")]
	StdErr,
}

pub struct TailOpts<'a> {
	pub environment: &'a str,
	pub actor_id: Uuid,
	pub stream: LogStream,
	pub follow: bool,
	pub timestamps: bool,
}

pub async fn tail(ctx: &toolchain::ToolchainCtx, opts: TailOpts<'_>) -> Result<()> {
	let mut watch_index: Option<String> = None;

	let stream = match opts.stream {
		LogStream::StdOut => models::ActorLogStream::StdOut,
		LogStream::StdErr => models::ActorLogStream::StdErr,
	};

	loop {
		let res = apis::actor_logs_api::actor_logs_get(
			&ctx.openapi_config_cloud,
			&opts.actor_id.to_string(),
			stream,
			Some(&ctx.project.name_id),
			Some(opts.environment),
			watch_index.as_deref(),
		)
		.await
		.map_err(|err| anyhow!("Failed to fetch logs: {err}"))?;
		watch_index = Some(res.watch.index);

		if !opts.follow {
			break;
		}

		for (ts, line) in res.timestamps.iter().zip(res.lines.iter()) {
			let decoded_line = match STANDARD.decode(line) {
				Result::Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
				Err(_) => {
					eprintln!("Failed to decode base64: {line}");
					continue;
				}
			};

			if opts.timestamps {
				println!("{ts} {decoded_line}");
			} else {
				println!("{decoded_line}");
			}
		}
	}

	Ok(())
}
