use serde::Deserialize;

use crate::error::Error;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Cdn {
	pub site: String,
	pub routes: Vec<Route>,
}

#[derive(Debug, Deserialize)]
pub struct Route {
	pub glob: String,
	pub priority: u32,
	pub middlewares: Vec<middleware::Middleware>,
}

pub mod middleware {
	use serde::Deserialize;

	#[derive(Debug, Deserialize)]
	pub struct Middleware {
		pub kind: MiddlewareKind,
	}

	#[derive(Debug, Deserialize)]
	pub enum MiddlewareKind {
		CustomHeaders {
			headers: Vec<custom_headers::Header>,
		},
	}

	pub mod custom_headers {
		use serde::Deserialize;

		#[derive(Debug, Deserialize)]
		pub struct Header {
			pub name: String,
			pub value: String,
		}
	}
}

impl Cdn {
	pub fn build_model(
		&self,
		_game: &rivet_cloud::model::GameFull,
	) -> Result<rivet_cloud::model::CdnVersionConfig, Error> {
		use rivet_cloud::model::*;

		let routes = self
			.routes
			.iter()
			.map(|route| {
				Ok(CdnVersionRoute::builder()
					.glob(route.glob.clone())
					.priority(route.priority as i32)
					.set_middlewares(Some(
						route
							.middlewares
							.iter()
							.map(|middleware| {
								let kind = match &middleware.kind {
									middleware::MiddlewareKind::CustomHeaders { headers } => {
										CdnVersionMiddlewareKind::CustomHeaders(
											CdnVersionCustomHeadersMiddleware::builder()
												.set_headers(Some(
													headers
														.iter()
														.map(|header| {
															CdnVersionHeader::builder()
																.name(&header.name)
																.value(&header.value)
																.build()
														})
														.collect::<Vec<_>>(),
												))
												.build(),
										)
									}
								};

								Ok(CdnVersionMiddleware::builder().kind(kind).build())
							})
							.collect::<Result<Vec<_>, Error>>()?,
					))
					.build())
			})
			.collect::<Result<Vec<_>, Error>>()?;

		Ok(CdnVersionConfig::builder()
			.site_id(&self.site)
			.set_routes(Some(routes))
			.build())
	}
}
