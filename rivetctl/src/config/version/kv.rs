use serde::Deserialize;

use crate::error::Error;

#[derive(Debug, Deserialize)]
pub struct Kv {}

impl Kv {
	pub fn build_model(
		self,
		game: &rivet_cloud::model::GameFull,
	) -> Result<rivet_cloud::model::KvVersionConfig, Error> {
		use rivet_cloud::model::*;

		Ok(KvVersionConfig::builder().build())
	}
}
