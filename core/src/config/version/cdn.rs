use std::collections::HashMap;

use serde::Deserialize;

use crate::error::Error;

#[derive(Debug, Deserialize)]
pub struct Cdn {
	pub site: String,
	pub custom_headers: Vec<custom_header::CustomHeader>,
}

pub mod custom_header {
	use serde::Deserialize;

	#[derive(Debug, Deserialize)]
	pub struct CustomHeader {
		pub glob: String,
		pub priority: u32,
		pub headers: Vec<Header>,
	}

	#[derive(Debug, Deserialize)]
	pub struct Header {
		pub name: String,
		pub value: String,
	}
}

impl Cdn {
	pub fn build_model(
		&self,
		_game: &rivet_cloud::model::GameFull,
	) -> Result<rivet_cloud::model::CdnVersionConfig, Error> {
		use rivet_cloud::model::*;

		let custom_headers = self
			.custom_headers
			.iter()
			.map(|custom_header| {
				Ok(CdnVersionCustomHeader::builder()
					.glob(custom_header.glob.clone())
					.priority(custom_header.priority as i32)
					.set_headers(Some(
						custom_header
							.headers
							.iter()
							.map(|header| (header.name.clone(), header.value.clone()))
							.collect::<HashMap<_, _>>(),
					))
					.build())
			})
			.collect::<Result<Vec<_>, Error>>()?;

		Ok(CdnVersionConfig::builder()
			.site_id(&self.site)
			.set_custom_headers(Some(custom_headers))
			.build())
	}
}
