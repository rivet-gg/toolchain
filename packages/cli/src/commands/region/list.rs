use anyhow::*;
use clap::Parser;
use toolchain::rivet_api::apis;

#[derive(Parser)]
pub struct Opts {
	#[clap(long, alias = "env", short = 'e')]
	environment: Option<String>,
}

impl Opts {
	pub async fn execute(&self) -> Result<()> {
		let ctx = toolchain::toolchain_ctx::load().await?;

		let env = crate::util::env::get_or_select(&ctx, self.environment.as_ref()).await?;

		let res = apis::actor_regions_api::actor_regions_list(
			&ctx.openapi_config_cloud,
			Some(&ctx.project.name_id),
			Some(&env),
		)
		.await
		.context("Failed to list regions")?;

		println!("{:#?}", res.regions);
		Ok(())
	}
}
