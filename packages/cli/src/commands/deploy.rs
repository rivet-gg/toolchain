use clap::Parser;
use std::{collections::HashMap, process::ExitCode};

#[derive(Parser)]
pub struct Opts {
	#[clap(long, alias = "env", short = 'e')]
	environment: Option<String>,

	#[clap(long, short = 't')]
	tags: Option<String>,
}

impl Opts {
	pub async fn execute(&self) -> ExitCode {
		let ctx = match toolchain::toolchain_ctx::load().await {
			Ok(c) => c,
			Err(err) => {
				eprintln!("Failed to load ctx: {err:?}");
				return ExitCode::FAILURE;
			}
		};

		let env = match crate::util::env::get_or_select(&ctx, self.environment.as_ref()).await {
			Ok(e) => e,
			Err(err) => {
				eprintln!("Failed to select env: {err:?}");
				return ExitCode::FAILURE;
			}
		};

		let build_tags = match self
			.tags
			.as_ref()
			.map(|b| kv_str::from_str::<HashMap<String, String>>(b))
			.transpose()
		{
			Ok(t) => t,
			Err(err) => {
				eprintln!("Failed to parse build tags: {err:?}");
				return ExitCode::FAILURE;
			}
		};

		match crate::util::deploy::deploy(crate::util::deploy::DeployOpts {
			environment: &env,
			build_tags,
		})
		.await
		{
			Ok(_) => ExitCode::SUCCESS,
			Err(code) => code,
		}
	}
}
