use std::sync::Arc;

use crate::{config::Config, error::Error};

pub type Ctx = Arc<SharedCtx>;

pub struct SharedCtx {
	config: Config,
	base_path: Option<String>,
}

impl SharedCtx {
	pub async fn new(config: Config, base_path: Option<String>) -> Result<Ctx, Error> {
		Ok(Arc::new(SharedCtx { config, base_path }))
	}

	/// Retrieves a token to use with the API.
	fn token(&self) -> Result<&str, Error> {
		self.config
			.auth
			.token
			.as_ref()
			.map(String::as_str)
			.ok_or(Error::NotAuthenticated)
	}

	pub fn api_config(&self) -> Result<rivet_cloud::apis::configuration::Configuration, Error> {
		let mut config = rivet_cloud::apis::configuration::Configuration {
			user_agent: Some(format!(
				"{}/{}",
				env!("CARGO_PKG_NAME"),
				env!("CARGO_PKG_VERSION")
			)),
			bearer_access_token: Some(self.token()?.to_owned()),
			..Default::default()
		};
		if let Some(base_path) = &self.base_path {
			config.base_path = base_path.clone();
		}
		Ok(config)
	}
}
