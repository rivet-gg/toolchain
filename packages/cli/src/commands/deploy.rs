use clap::Parser;
use std::{collections::HashMap, process::ExitCode};

#[derive(Parser)]
pub struct Opts {
	environment: String,

	#[clap(long, short = 't')]
	tags: Option<String>,
}

impl Opts {
	pub async fn execute(&self) -> ExitCode {
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
			environment: &self.environment,
			build_tags,
		})
		.await
		{
			Ok(_) => ExitCode::SUCCESS,
			Err(code) => code,
		}
	}
}
