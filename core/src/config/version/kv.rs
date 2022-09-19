use serde::Deserialize;

use crate::error::Error;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Kv {}

impl Kv {
	pub fn build_model(
		&self,
		_game: &rivet_cloud::model::GameFull,
	) -> Result<rivet_cloud::model::KvVersionConfig, Error> {
		use rivet_cloud::model::*;

		Ok(KvVersionConfig::builder().build())
	}
}
