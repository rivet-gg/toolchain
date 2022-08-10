use std::sync::Arc;

use crate::{config::global::Config as GlobalConfig, error::Error};

pub type Ctx = Arc<CtxInner>;

pub struct CtxInner {
	pub config: GlobalConfig,
	pub http_client:
		rivet_cloud::Client<aws_smithy_client::erase::DynConnector, tower::layer::util::Identity>,
	pub concurrent_uploads: usize,
	pub override_api_url: Option<String>,
	pub override_access_token: Option<String>,
}

pub async fn init(
	config: GlobalConfig,
	override_api_url: Option<String>,
	override_access_token: Option<String>,
) -> Result<Ctx, Error> {
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
		.set_bearer_token(
			override_access_token
				.clone()
				.or_else(|| config.auth.token.clone())
				.ok_or(Error::NotAuthenticated)?
				.to_owned(),
		)
		.build();
	let http_client = rivet_cloud::Client::with_config(raw_client, rivet_cloud_config);

	Ok(Arc::new(CtxInner {
		config,
		http_client,
		concurrent_uploads: 8,
		override_api_url,
		override_access_token,
	}))
}
