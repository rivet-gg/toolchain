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
	let rivet_cloud_config = rivet_cloud::Config::builder()
		.set_uri(
			override_api_url
				.clone()
				.unwrap_or_else(|| "https://cloud.api.rivet.gg/v1".to_string()),
		)
		.set_bearer_token(access_token.clone())
		.build();
	let http_client = rivet_cloud::Client::with_config(raw_client, rivet_cloud_config);

	Ok(Arc::new(CtxInner {
		http_client,
		concurrent_uploads: 8,
		override_api_url,
		access_token,
	}))
}
