use std::io::Write;

use clap::Parser;
use cli_core::rivet_api::{apis, models};
use global_error::prelude::*;
use serde::Serialize;
use std::collections::HashMap;

use crate::{commands, util::global_config};

#[derive(Parser)]
pub struct Opts {
	/// Namespace to create token for
	#[clap(short = 'n', long)]
	pub namespace: Option<String>,
}

impl Opts {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> GlobalResult<()> {
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

pub async fn execute(ctx: &cli_core::Ctx, opts: &Opts) -> GlobalResult<Output> {
	let ns_name_id = opts
		.namespace
		.as_ref()
		.map(String::as_str)
		.unwrap_or("staging");

	// Read dev config properties
	let (dev_hostname, dev_ports) = read_config(ns_name_id).await?;

	// Attempt to find existing token
	let existing_token = global_config::read_project(|config| {
		config
			.tokens
			.development
			.iter()
			.find(|t| {
				t.namespace_name_id == ns_name_id
					&& t.hostname == dev_hostname
					&& t.ports == dev_ports
			})
			.map(|x| x.token.clone())
	})
	.await?;
	if let Some(token) = existing_token {
		return Ok(Output { token });
	}

	// Create dev token
	let namespace_id = fetch_namespace_id(&ctx, &ns_name_id).await?;
	let token_res = apis::cloud_games_namespaces_api::cloud_games_namespaces_create_game_namespace_token_development(
			&ctx.openapi_config_cloud,
			&ctx.game_id,
			&namespace_id,
			models::CloudGamesNamespacesCreateGameNamespaceTokenDevelopmentRequest {
				hostname: dev_hostname.clone(),
				ports: Some(dev_ports.clone()),
				lobby_ports: None,
			},
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
			.development
			.push(global_config::DevelopmentToken {
				namespace_name_id: ns_name_id.to_owned(),
				hostname: dev_hostname,
				ports: dev_ports,
				token: token.clone(),
			});
	})
	.await?;

	Ok(Output { token })
}

async fn read_config(
	ns_name_id: &str,
) -> GlobalResult<(
	String,
	HashMap<String, models::CloudMatchmakerDevelopmentPort>,
)> {
	let config = commands::config::read_config(Vec::new(), Some(ns_name_id)).await?;

	if let Some(matchmaker) = &config.matchmaker {
		let dev_hostname = matchmaker
			.dev_hostname
			.clone()
			.unwrap_or_else(|| "127.0.0.1".to_string());

		// Read lobby ports that we need to expose
		let dev_ports = {
			let mut lobby_ports =
				HashMap::<String, &models::CloudVersionMatchmakerGameModeRuntimeDockerPort>::new();

			// Register lobby ports from all game modes
			if let Some(game_modes) = matchmaker.game_modes.as_ref() {
				for game_mode in game_modes.values() {
					if let Some(ports) = game_mode.docker.as_ref().and_then(|x| x.ports.as_ref()) {
						for (label, port) in ports {
							lobby_ports.insert(label.clone(), port);
						}
					}
				}
			}

			// Global overrides take priority because we don't know what game
			// mode will be running locally
			if let Some(ports) = matchmaker.docker.as_ref().and_then(|x| x.ports.as_ref()) {
				for (label, port) in ports {
					lobby_ports.insert(label.clone(), port);
				}
			}

			lobby_ports
		};

		let dev_ports = dev_ports
			.into_iter()
			.map(|(label, port_config)| {
				//
				let (port, port_range) = if let Some(dev_port) = port_config.dev_port {
					(Some(dev_port), None)
				} else if let Some(dev_port_range) = port_config.dev_port_range.as_ref() {
					(None, Some(dev_port_range.clone()))
				} else if let Some(port) = port_config.port {
					(Some(port), None)
				} else if let Some(port_range) = port_config.port_range.as_ref() {
					(None, Some(port_range.clone()))
				} else {
					bail!("missing both port and port_range")
				};

				Ok((
					label,
					models::CloudMatchmakerDevelopmentPort {
						port,
						port_range,
						protocol: port_config
							.dev_protocol
							// Default to non-TLS version of the given protocol
							.unwrap_or({
								use models::CloudVersionMatchmakerPortProtocol::*;
								match port_config.protocol {
									Some(Https) | Some(Http) | None => Http,
									Some(Tcp) | Some(TcpTls) => Tcp,
									Some(Udp) => Udp,
								}
							}),
					},
				))
			})
			.collect::<GlobalResult<HashMap<_, _>>>()?;

		Ok((dev_hostname, dev_ports))
	} else {
		Ok((
			"127.0.0.1".to_string(),
			HashMap::<String, models::CloudMatchmakerDevelopmentPort>::new(),
		))
	}
}

async fn fetch_namespace_id(ctx: &cli_core::Ctx, ns_name_id: &str) -> GlobalResult<String> {
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
		"no namespace for name id {}",
		ns_name_id
	);

	Ok(namespace_id)
}
