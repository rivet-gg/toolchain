use std::sync::Arc;

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

	// Create client
	let rivet_cloud_config = rivet_cloud::Config::builder()
		.set_uri(
			override_api_url
				.clone()
				.unwrap_or_else(|| "https://cloud.api.rivet.gg/v1".to_string()),
		)
		.set_bearer_token(access_token.clone())
		.build();
	let http_client = rivet_cloud::Client::with_config(raw_client, rivet_cloud_config);

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

	Ok(Arc::new(CtxInner {
		http_client,
		concurrent_uploads: 8,
		override_api_url,
		access_token,
		game_id,
	}))
}
