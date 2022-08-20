use serde::Deserialize;

use crate::error::Error;

pub mod cdn;
pub mod kv;
pub mod mm;

#[derive(Debug, Deserialize)]
pub struct Version {
	#[serde(default)]
	pub cdn: Option<cdn::Cdn>,
	#[serde(default)]
	pub matchmaker: Option<mm::Matchmaker>,
	#[serde(default)]
	pub kv: Option<kv::Kv>,
}

impl Version {
	pub fn build_model(
		&self,
		game: &rivet_cloud::model::GameFull,
	) -> Result<rivet_cloud::model::CloudVersionConfig, Error> {
		use rivet_cloud::model::*;

		Ok(CloudVersionConfig::builder()
			.set_cdn(self.cdn.as_ref().map(|x| x.build_model(game)).transpose()?)
			.set_matchmaker(
				self.matchmaker
					.as_ref()
					.map(|x| x.build_model(game))
					.transpose()?,
			)
			.set_kv(self.kv.as_ref().map(|x| x.build_model(game)).transpose()?)
			.build())
	}
}
