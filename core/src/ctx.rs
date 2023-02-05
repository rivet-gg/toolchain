use std::{env, sync::Arc};

use crate::error::Error;

pub type Ctx = Arc<CtxInner>;

type HttpClient =
	rivet_cloud::Client<aws_smithy_client::erase::DynConnector, tower::layer::util::Identity>;

pub struct CtxInner {
	http_client: HttpClient,
	pub concurrent_uploads: usize,
	pub override_api_url: Option<String>,
	pub access_token: String,
	pub game_id: String,

	pub openapi_config_cloud: rivet_api::apis::configuration::Configuration,
}

impl CtxInner {
	pub fn client(&self) -> &HttpClient {
		&self.http_client
	}
}

pub async fn init(override_api_url: Option<String>, access_token: String) -> Result<Ctx, Error> {
	let raw_client = rivet_cloud::Builder::dyn_https()
		.middleware(tower::layer::util::Identity::new())
		.sleep_impl(None)
		.build();

	let uri = override_api_url
		.clone()
		.unwrap_or_else(|| "https://cloud.api.rivet.gg/v1".to_string());

	// Create client
	let rivet_cloud_config = rivet_cloud::Config::builder()
		.set_uri(uri.clone())
		.set_bearer_token(access_token.clone())
		.build();
	let http_client = rivet_cloud::Client::with_config(raw_client, rivet_cloud_config);

	// Create OpenAPI config
	let openapi_config_cloud = rivet_api::apis::configuration::Configuration {
		base_path: uri.clone(),
		bearer_access_token: Some(access_token.clone()),
		..Default::default()
	};

	// Inspect token
	let inspect = http_client
		.inspect()
		.send()
		.await
		.map_err(|source| Error::InspectFail { source })?;
	let game_id = if let crate::rivet_cloud::model::AuthAgent::GameCloud(game_cloud) =
		inspect.agent.as_ref().ok_or_else(|| Error::Internal {
			message: "inspect.agent".into(),
		})? {
		game_cloud.game_id.clone().ok_or_else(|| Error::Internal {
			message: "game_cloud.game_id".into(),
		})?
	} else {
		return Err(Error::InvalidAgentKind);
	};

	let concurrent_uploads = env::var("RIVET_CONCURRENT_UPLOADS")
		.ok()
		.and_then(|x| x.parse::<usize>().ok())
		.unwrap_or(8);

	Ok(Arc::new(CtxInner {
		http_client,
		concurrent_uploads,
		override_api_url,
		access_token,
		game_id,

		openapi_config_cloud,
	}))
}
