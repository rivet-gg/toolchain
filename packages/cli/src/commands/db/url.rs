use clap::Parser;
use serde::Serialize;
use std::process::ExitCode;

use crate::util::postgres;

/// Print database URL
#[derive(Parser, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Opts {
	#[clap(long)]
	database: Option<String>,
}

impl Opts {
	pub async fn execute(&self) -> ExitCode {
		let Ok(postgres) = postgres::ensure_running().await else {
			return ExitCode::FAILURE;
		};

		let db = self
			.database
			.as_ref()
			.map(String::as_str)
			.unwrap_or("postgres");
		println!("{}", postgres.url(db).await);

		ExitCode::SUCCESS
	}
}
