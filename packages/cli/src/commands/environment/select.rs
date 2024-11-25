use anyhow::*;
use clap::Parser;
use std::process::ExitCode;

#[derive(Parser)]
pub struct Opts {}

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
		crate::util::env::select(&ctx).await?;
		Ok(ExitCode::SUCCESS)
	}
}
