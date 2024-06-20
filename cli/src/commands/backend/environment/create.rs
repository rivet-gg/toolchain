use clap::Parser;
use cli_core::rivet_api::{apis, models};
use global_error::prelude::*;

#[derive(Parser)]
pub struct Opts {
	#[clap(index = 1)]
	display_name: String,

	#[clap(index = 2)]
	name_id: String,

	/// Tier of the new environment. ("shared", "dedicated")
	#[clap(long, default_value = "shared")]
	tier: Tier,
}

impl Opts {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> GlobalResult<()> {
		let project_res = apis::ee_cloud_games_projects_api::ee_cloud_games_projects_get(
			&ctx.openapi_config_cloud,
			&ctx.game_id,
		)
		.await?;

		// TODO: Add link to dashboard to this error message
		let project = unwrap!(
			project_res.project,
			"no OpenGB project linked to the current game. Create one on the dashboard."
		);

		rivet_term::status::info("Creating environment", &self.display_name);

		apis::ee_cloud_backend_projects_envs_api::ee_cloud_backend_projects_envs_create(
			&ctx.openapi_config_cloud,
			project.project_id.to_string().as_str(),
			models::EeCloudBackendProjectsEnvsCreateRequest {
				display_name: self.display_name.clone(),
				name_id: self.name_id.clone(),
				tier: self.tier.into(),
			},
		)
		.await?;

		rivet_term::status::success("Done", "");

		Ok(())
	}
}

#[derive(Copy, Clone, strum::EnumString, strum::AsRefStr)]
enum Tier {
	#[strum(serialize = "shared")]
	Shared,
	#[strum(serialize = "dedicated")]
	Dedicated,
}

impl From<Tier> for models::EeBackendTier {
	fn from(value: Tier) -> Self {
		match value {
			Tier::Shared => models::EeBackendTier::Shared,
			Tier::Dedicated => models::EeBackendTier::Dedicated,
		}
	}
}
