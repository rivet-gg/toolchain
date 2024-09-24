use clap::Parser;
use serde::Serialize;
use std::process::{ExitCode, Stdio};
use tokio::process::Command;

use crate::util::postgres;

/// Open shell to query database
#[derive(Parser, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Opts {}

impl Opts {
	pub async fn execute(&self) -> ExitCode {
		let Ok(postgres) = postgres::ensure_running().await else {
			return ExitCode::FAILURE;
		};

		// Spawn psql
		let psql_path = postgres.bin_dir().await.join("psql");
		let status = Command::new(psql_path)
			.arg(postgres.url("postgres").await)
			.stdin(Stdio::inherit())
			.stdout(Stdio::inherit())
			.stderr(Stdio::inherit())
			.kill_on_drop(true)
			.status()
			.await;
		match status {
			Ok(x) if x.success() => ExitCode::SUCCESS,
			Ok(_) => ExitCode::FAILURE,
			Err(err) => {
				eprintln!("Failed to spawn psql: {err:?}");
				ExitCode::FAILURE
			}
		}
	}
}
