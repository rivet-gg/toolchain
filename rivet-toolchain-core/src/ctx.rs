use rivet_api::apis;
use std::{env, sync::Arc};

use crate::error::Error;

pub const VERSION: &str = concat!(
	env!("VERGEN_BUILD_SEMVER"),
	" (",
	env!("VERGEN_GIT_SHA_SHORT"),
	")"
);

pub fn user_agent() -> String {
	format!("CLI/{VERSION}")
}

pub const DEFAULT_API_ENDPOINT: &'static str = "https://api.rivet.gg";

pub type Ctx = Arc<CtxInner>;

pub struct CtxInner {
	pub api_endpoint: String,
	pub access_token: String,
	pub game_id: String,

	/// Domains that host parts of Rivet
	pub bootstrap: rivet_api::models::CloudBootstrapResponse,

	pub openapi_config_cloud: apis::configuration::Configuration,
}

pub async fn init(api_endpoint: Option<String>, access_token: String) -> Result<Ctx, Error> {
	let api_endpoint = api_endpoint
		.clone()
		.unwrap_or_else(|| DEFAULT_API_ENDPOINT.to_string());

	// Create OpenAPI config
	let openapi_config_cloud = apis::configuration::Configuration {
		base_path: api_endpoint.clone(),
		bearer_access_token: Some(access_token.clone()),
		user_agent: Some(user_agent()),
		..Default::default()
	};

	// Make requests
	let (inspect_response, bootstrap_response): (
		rivet_api::models::CloudInspectResponse,
		rivet_api::models::CloudBootstrapResponse,
	) = tokio::try_join!(
		async {
			apis::cloud_auth_api::cloud_auth_inspect(&openapi_config_cloud)
				.await
				.map_err(|e| Error::InspectFail { source: e })
		},
		async {
			apis::cloud_api::cloud_bootstrap(&openapi_config_cloud)
				.await
				.map_err(|e| Error::BootstrapFail { source: e })
		}
	)?;

	let game_id = if let Some(game_cloud) = inspect_response.agent.game_cloud {
		game_cloud.game_id
	} else {
		return Err(Error::InvalidAgentKind);
	};

	Ok(Arc::new(CtxInner {
		api_endpoint,
		access_token,
		game_id: game_id.to_string(),
		bootstrap: bootstrap_response,
		openapi_config_cloud,
	}))
}
