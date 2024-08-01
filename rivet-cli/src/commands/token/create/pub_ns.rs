use std::io::Write;

use clap::Parser;
use toolchain_core::rivet_api::apis;
use global_error::prelude::*;
use serde::Serialize;

use crate::util::global_config;

#[derive(Parser)]
pub struct Opts {
	/// Namespace to create token for
	#[clap(short = 'n', long)]
	pub namespace: Option<String>,
}

impl Opts {
	pub async fn execute(&self, ctx: &toolchain_core::Ctx) -> GlobalResult<()> {
		let output = execute(ctx, self).await?;

		print!("{}", output.token);
		std::io::stdout().flush()?;
		eprintln!();

		Ok(())
	}
}

#[derive(Serialize)]
pub struct Output {
	pub token: String,
}

pub async fn execute(ctx: &toolchain_core::Ctx, opts: &Opts) -> GlobalResult<Output> {
	let ns_name_id = opts
		.namespace
		.as_ref()
		.map(String::as_str)
		.unwrap_or("staging");

	// Attempt to find existing token
	let existing_token = global_config::read_project(|config| {
		config
			.tokens
			.public_namespace
			.iter()
			.find(|t| t.namespace_name_id == ns_name_id)
			.map(|x| x.token.clone())
	})
	.await?;
	if let Some(token) = existing_token {
		return Ok(Output { token });
	}

	// Create token
	let namespace_id = fetch_namespace_id(&ctx, &ns_name_id).await?;
	let token_res = apis::cloud_games_namespaces_api::cloud_games_namespaces_create_game_namespace_token_public(
			&ctx.openapi_config_cloud,
			&ctx.game_id,
			&namespace_id,
		)
		.await;
	if let Err(err) = token_res.as_ref() {
		println!("Error: {err:?}");
	}
	let token_res = unwrap!(token_res);
	let token = token_res.token;

	// Save token
	global_config::mutate_project(|config| {
		config
			.tokens
			.public_namespace
			.push(global_config::PublicNamespaceToken {
				namespace_name_id: ns_name_id.to_owned(),
				token: token.clone(),
			});
	})
	.await?;

	Ok(Output { token })
}

async fn fetch_namespace_id(ctx: &toolchain_core::Ctx, ns_name_id: &str) -> GlobalResult<String> {
	let game_res = unwrap!(
		apis::cloud_games_api::cloud_games_get_game_by_id(
			&ctx.openapi_config_cloud,
			&ctx.game_id,
			None,
		)
		.await
	);
	let namespace_id = unwrap!(
		game_res
			.game
			.namespaces
			.iter()
			.find(|x| x.name_id == ns_name_id)
			.map(|x| x.namespace_id.to_string()),
		"no namespace for name id"
	);

	Ok(namespace_id)
}
