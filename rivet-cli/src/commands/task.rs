use clap::Parser;
use global_error::prelude::*;

/// EXPERIMENTAL
#[derive(Parser)]
pub enum SubCommand {
	Run(RunOpts),
}

impl SubCommand {
	pub async fn execute(&self) -> GlobalResult<()> {
		match self {
			SubCommand::Run(opts) => opts.execute().await,
		}
	}
}

#[derive(Parser)]
pub struct RunOpts {
	#[clap(long)]
	run_config: String,
	#[clap(long)]
	name: String,
	#[clap(long)]
	input: String,
}

impl RunOpts {
	pub async fn execute(&self) -> GlobalResult<()> {
		let run_config = serde_json::from_str(&self.run_config)?;
		let output = toolchain::tasks::run_task_json(run_config, &self.name, &self.input).await;
		println!("{output}");
		Ok(())
	}
}
