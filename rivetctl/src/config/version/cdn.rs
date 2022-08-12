use serde::Deserialize;

use crate::error::Error;

#[derive(Debug, Deserialize)]
pub struct Cdn {
	pub site: String,
}

impl Cdn {
	pub fn build_model(
		self,
		_game: &rivet_cloud::model::GameFull,
	) -> Result<rivet_cloud::model::CdnVersionConfig, Error> {
		use rivet_cloud::model::*;

		Ok(CdnVersionConfig::builder().build())
	}
}
