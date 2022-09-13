use anyhow::{Context, Result};
use clap::Parser;
use console::Term;
use tokio::fs;

use crate::util::term;

#[derive(Parser)]
pub enum SubCommand {
	Init,
}

impl SubCommand {
	pub async fn execute(&self, term: &Term, ctx: &cli_core::Ctx) -> Result<()> {
		match self {
			SubCommand::Init => {
				let game_res = ctx
					.client()
					.get_game_by_id()
					.game_id(&ctx.game_id)
					.send()
					.await
					.context("client.get_game_by_id")?;
				let game = game_res.game.context("game_res.game")?;
				let game_ns = game.namespaces().context("game.namespaces")?;
				let prod_namespace_id = game_ns
					.iter()
					.find(|x| x.name_id().map_or(false, |x| x == "prod"))
					.and_then(|x| x.namespace_id())
					.context("game.namespaces.find(\"prod\").namespace_id")?;

				eprintln!();
				let hostname =
					term::input::string_with_tip(term, "Dev hostname?", "example: 127.0.0.1")
						.await?;

				let mut default_port = Option::<u16>::None;
				let mut lobby_ports = Vec::new();
				loop {
					eprintln!();

					let label = term::input::string_with_tip(
						term,
						"Development port label?",
						"example: default",
					)
					.await?;

					let proto = 'proto: loop {
						let proto = term::input::string_with_tip(
							term,
							"Dev port protocol?",
							"http/https/udp, usually: http",
						)
						.await?;
						match proto.to_lowercase().as_str() {
							"http" => {
								break 'proto cli_core::rivet_cloud::model::ProxyProtocol::Http;
							}
							"https" => {
								break 'proto cli_core::rivet_cloud::model::ProxyProtocol::Https;
							}
							"udp" => {
								break 'proto cli_core::rivet_cloud::model::ProxyProtocol::Udp;
							}
							_ => {
								term::status::error("Invalid protocol", "");
								eprintln!();
							}
						}
					};

					let (target_port, port_range) = match proto {
						cli_core::rivet_cloud::model::ProxyProtocol::Http
						| cli_core::rivet_cloud::model::ProxyProtocol::Https => {
							// Create new port
							let port = 'port: loop {
								let port = term::input::string_with_tip(
									term,
									"Dev port?",
									"0-65535, example: 8080",
								)
								.await?;
								if let Ok(port) = port.parse::<u16>() {
									break 'port port;
								} else {
									term::status::error("Invalid port number", "");
									eprintln!();
								}
							};

							// Assign default port
							if default_port.is_none() {
								default_port = Some(port);
							}

							(Some(port), None)
						}
						cli_core::rivet_cloud::model::ProxyProtocol::Udp => {
							// Prompt min port
							let port_min = 'port: loop {
								let port = term::input::string_with_tip(
									term,
									"Dev port range min?",
									"26000-31999",
								)
								.await?;
								if let Ok(port) = port.parse::<u16>() {
									break 'port port;
								} else {
									term::status::error("Invalid port number", "");
									eprintln!();
								}
							};

							// Prompt max port
							let port_max = 'port: loop {
								let port = term::input::string_with_tip(
									term,
									"Dev port range max?",
									"26000-31999",
								)
								.await?;
								if let Ok(port) = port.parse::<u16>() {
									break 'port port;
								} else {
									term::status::error("Invalid port number", "");
									eprintln!();
								}
							};

							(
								None,
								Some(
									cli_core::rivet_cloud::model::PortRange::builder()
										.min(port_min as i32)
										.max(port_max as i32)
										.build(),
								),
							)
						}
						_ => {
							unreachable!("unrecognized protocol variant")
						}
					};

					lobby_ports.push(
						cli_core::rivet_cloud::model::LobbyGroupRuntimeDockerPort::builder()
							.label(&label)
							.set_target_port(target_port.map(|x| x as i32))
							.set_port_range(port_range.clone())
							.proxy_protocol(proto.clone())
							.build(),
					);

					term::status::success(
						format!("Added {label}"),
						format!(
							"{port} proto={proto:?}",
							port = if let Some(x) = target_port {
								format!("target_port={x}")
							} else if let Some(port_range) = &port_range {
								format!(
									"port_range={}-{}",
									port_range.min.unwrap(),
									port_range.max.unwrap()
								)
							} else {
								unreachable!("missing both target_port and port_range")
							}
						),
					);

					if !term::input::bool(term, "Add another port?").await? {
						break;
					}
				}

				let token_res = ctx
					.client()
					.create_game_namespace_token_development()
					.game_id(&ctx.game_id)
					.namespace_id(prod_namespace_id)
					.hostname(hostname)
					.set_lobby_ports(Some(lobby_ports.clone()))
					.send()
					.await
					.context("client.create_game_namespace_token_development")?;
				let token = token_res.token().context("token_res.token")?;

				eprintln!();
				term::status::success(format!("Token created"), "");

				eprintln!();
				if term::input::bool(term, "Write token to .env file?").await? {
					let mut env_file =
						format!("RIVET_CLIENT_TOKEN={token}\nRIVET_LOBBY_TOKEN={token}\n");

					if let Some(default_port) = default_port {
						env_file.push_str(&format!("PORT={default_port}\n"));
					}

					for port in &lobby_ports {
						let label = port.label.as_ref().unwrap();

						if let Some(port) = port.target_port {
							env_file.push_str(&format!("PORT_{label}={port}\n"));
						} else if let Some(port_range) = &port.port_range {
							env_file.push_str(&format!(
								"PORT_RANGE_MIN_{label}={min}\nPORT_RANGE_MAX_{label}={max}\n",
								min = port_range.min.unwrap(),
								max = port_range.max.unwrap(),
							));
						}
					}

					fs::write(".env", env_file).await?;
					term::status::success(format!("Wrote to .env"), "");
				} else {
					println!("{token}");
				}

				Ok(())
			}
		}
	}
}
