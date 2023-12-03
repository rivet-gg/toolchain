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

pub const DEFAULT_API_CLOUD_URL: &'static str = "https://cloud.api.rivet.gg/v1";

pub type Ctx = Arc<CtxInner>;

pub struct CtxInner {
	pub concurrent_uploads: usize,
	pub override_api_url: Option<String>,
	pub access_token: String,
	pub game_id: String,

	pub openapi_config_cloud: rivet_api::apis::configuration::Configuration,
}

pub async fn init(override_api_url: Option<String>, access_token: String) -> Result<Ctx, Error> {
	let uri = override_api_url
		.clone()
		.unwrap_or_else(|| DEFAULT_API_CLOUD_URL.to_string());

	// Create OpenAPI config
	let openapi_config_cloud = rivet_api::apis::configuration::Configuration {
		base_path: uri.clone(),
		bearer_access_token: Some(access_token.clone()),
		user_agent: Some(user_agent()),
		..Default::default()
	};

	// Inspect token
	let inspect = rivet_api::apis::cloud_auth_api::cloud_auth_inspect(&openapi_config_cloud)
		.await
		.map_err(|source| Error::InspectFail { source })?;
	let game_id = if let Some(game_cloud) = inspect.agent.game_cloud {
		game_cloud.game_id
	} else {
		return Err(Error::InvalidAgentKind);
	};

	let concurrent_uploads = env::var("RIVET_CONCURRENT_UPLOADS")
		.ok()
		.and_then(|x| x.parse::<usize>().ok())
		.unwrap_or(8);

	Ok(Arc::new(CtxInner {
		concurrent_uploads,
		override_api_url,
		access_token,
		game_id: game_id.to_string(),

		openapi_config_cloud,
	}))
}
