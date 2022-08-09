use std::sync::Arc;

use crate::{config::Config, error::Error};

pub type Ctx = Arc<SharedCtx>;

pub struct SharedCtx {
	pub http_client:
		rivet_cloud::Client<aws_smithy_client::erase::DynConnector, tower::layer::util::Identity>,
	pub concurrent_uploads: usize,
}

impl SharedCtx {
	pub async fn new(
		config: Config,
		api_url: Option<String>,
		access_token: Option<String>,
	) -> Result<Ctx, Error> {
		let raw_client = rivet_cloud::Builder::dyn_https()
			.middleware(tower::layer::util::Identity::new())
			.sleep_impl(None)
			.build();
		let config = rivet_cloud::Config::builder()
			.set_uri(api_url.unwrap_or_else(|| "https://cloud.api.rivet.gg/v1".to_string()))
			.set_bearer_token(
				access_token
					.clone()
					.or_else(|| config.auth.token.clone())
					.ok_or(Error::NotAuthenticated)?
					.to_owned(),
			)
			.build();
		let http_client = rivet_cloud::Client::with_config(raw_client, config);

		Ok(Arc::new(SharedCtx {
			http_client,
			concurrent_uploads: 8,
		}))
	}
}
