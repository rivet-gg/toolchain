use clap::Parser;
use global_error::prelude::*;
use toolchain::{
	tasks::{unlink, RunConfig},
	util::task::run_task,
};

#[derive(Parser)]
pub struct Opts {}

impl Opts {
	pub async fn execute(&self) -> GlobalResult<()> {
		let (run_config, _temp_dir) = RunConfig::with_temp_dir()?;

		run_task::<unlink::Task>(run_config.clone(), unlink::Input {}).await?;
		eprintln!("Logged out");

		Ok(())
	}
}
