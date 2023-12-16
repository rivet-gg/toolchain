use clap::Parser;
use global_error::prelude::*;

use crate::util::{global_config, paths};

#[derive(Parser)]
pub struct Opts {}

impl Opts {
	pub async fn execute(&self, _ctx: &cli_core::Ctx) -> GlobalResult<()> {
		let project_root = paths::project_root()?;
		global_config::try_mutate_global(|config| {
			config.project_roots.remove(&project_root);
			Ok(())
		})
		.await?;
		Ok(())
	}
}
