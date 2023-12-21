use clap::Parser;
use cli_core::{ctx, rivet_api::apis};

use global_error::prelude::*;
use serde::Serialize;

use crate::util::global_config;

use super::SideKickHandler;

#[derive(Parser)]
pub struct Opts {}

#[derive(Serialize)]
pub struct Output {
	pub device_link_url: String,
	pub device_link_token: String,
}

impl SideKickHandler for Output {}

impl Opts {
	pub async fn execute(&self) -> GlobalResult<Output> {
		let (api_endpoint, _token) = unwrap!(
			global_config::read_project(|x| {
				(x.cluster.api_endpoint.clone(), x.tokens.cloud.clone())
			})
			.await
		);

		// Create OpenAPI configuration without bearer token to send link request
		let openapi_config_cloud_unauthed = apis::configuration::Configuration {
			base_path: api_endpoint
				.clone()
				.unwrap_or_else(|| ctx::DEFAULT_API_ENDPOINT.to_string()),
			user_agent: Some(ctx::user_agent()),
			..Default::default()
		};

		// Prepare the link
		let prepare_res = unwrap!(
			apis::cloud_devices_links_api::cloud_devices_links_prepare(
				&openapi_config_cloud_unauthed,
			)
			.await
		);

		Ok(Output {
			device_link_url: prepare_res.device_link_url,
			device_link_token: prepare_res.device_link_token,
		})
	}
}
