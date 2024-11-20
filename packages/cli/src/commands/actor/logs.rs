use anyhow::*;
use base64::{engine::general_purpose::STANDARD, Engine};
use clap::{Parser, ValueEnum};
use std::process::ExitCode;
use toolchain::rivet_api::{apis, models};
use uuid::Uuid;

#[derive(ValueEnum, Clone)]
enum LogStream {
	#[clap(name = "stdout")]
	StdOut,
	#[clap(name = "stderr")]
	StdErr,
}

#[derive(Parser)]
pub struct Opts {
	#[clap(index = 1)]
	environment: String,

	#[clap(index = 2)]
	stream: LogStream,

	#[clap(long)]
	id: String,

	#[clap(long)]
	no_timestamps: bool,

	#[clap(long)]
	no_follow: bool,
}

impl Opts {
	pub async fn execute(&self) -> ExitCode {
		match self.execute_inner().await {
			Result::Ok(code) => code,
			Err(err) => {
				eprintln!("{err}");
				ExitCode::FAILURE
			}
		}
	}

	pub async fn execute_inner(&self) -> Result<ExitCode> {
		let ctx = toolchain::toolchain_ctx::load().await?;

		let actor_id = Uuid::parse_str(&self.id).context("invalid id uuid")?;

		let mut watch_index: Option<String> = None;

		loop {
			let res = apis::actor_logs_api::actor_logs_get(
				&ctx.openapi_config_cloud,
				&actor_id.to_string(),
				match self.stream {
					LogStream::StdOut => models::ActorLogStream::StdOut,
					LogStream::StdErr => models::ActorLogStream::StdErr,
				},
				Some(&ctx.project.name_id),
				Some(&self.environment),
				watch_index.as_deref(),
			)
			.await
			.map_err(|err| anyhow!("Failed to fetch logs: {err}"))?;
			watch_index = Some(res.watch.index);

			if self.no_follow {
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

				if self.no_timestamps {
					println!("{decoded_line}");
				} else {
					println!("{ts} {decoded_line}");
				}
			}
		}

		Ok(ExitCode::SUCCESS)
	}
}
