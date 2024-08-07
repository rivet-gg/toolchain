use global_error::prelude::*;
use rivet_api::apis;
use std::{env, sync::Arc};

use crate::config;

pub const VERSION: &str = concat!(
	env!("VERGEN_BUILD_SEMVER"),
	" (",
	env!("VERGEN_GIT_SHA_SHORT"),
	")"
);

pub fn user_agent() -> String {
	format!("CLI/{VERSION}")
}

pub type Ctx = Arc<CtxInner>;

pub struct CtxInner {
	pub api_endpoint: String,
	pub access_token: String,
	pub game_id: String,

	/// Domains that host parts of Rivet
	pub bootstrap: rivet_api::models::CloudBootstrapResponse,

	pub openapi_config_cloud: apis::configuration::Configuration,
}

pub async fn load() -> GlobalResult<Ctx> {
	let (api_endpoint, token) =
		config::meta::read_project(|x| (x.cluster.api_endpoint.clone(), x.tokens.cloud.clone()))
			.await?;
	init(api_endpoint, token).await
}

pub async fn init(api_endpoint: String, cloud_token: String) -> GlobalResult<Ctx> {
	// Disable connection pooling to fix "connection closed before message completed"
	//
	// See https://github.com/hyperium/hyper/issues/2136#issuecomment-861826148
	let client = reqwest::Client::builder()
		.pool_max_idle_per_host(0)
		.build()?;

	// Create OpenAPI config
	let openapi_config_cloud = apis::configuration::Configuration {
		base_path: api_endpoint.clone(),
		bearer_access_token: Some(cloud_token.clone()),
		user_agent: Some(user_agent()),
		client,
		..Default::default()
	};

	// Make requests
	let (inspect_response, bootstrap_response): (
		rivet_api::models::CloudInspectResponse,
		rivet_api::models::CloudBootstrapResponse,
	) = tokio::try_join!(
		async {
			GlobalResult::Ok(unwrap!(
				apis::cloud_auth_api::cloud_auth_inspect(&openapi_config_cloud).await,
				"inspect failed"
			))
		},
		async {
			GlobalResult::Ok(unwrap!(
				apis::cloud_api::cloud_bootstrap(&openapi_config_cloud).await,
				"bootstrap failed"
			))
		}
	)?;

	let game_id = if let Some(game_cloud) = inspect_response.agent.game_cloud {
		game_cloud.game_id
	} else {
		bail!("invalid agent kind")
	};

	Ok(Arc::new(CtxInner {
		api_endpoint,
		access_token: cloud_token,
		game_id: game_id.to_string(),
		bootstrap: bootstrap_response,
		openapi_config_cloud,
	}))
}
