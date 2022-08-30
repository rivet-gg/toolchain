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

				let mut lobby_ports = Vec::new();
				loop {
					eprintln!();

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

					let proto = 'proto: loop {
						let proto = term::input::string_with_tip(
							term,
							"Dev port protocol?",
							"http/https, usually: http",
						)
						.await?;
						match proto.to_lowercase().as_str() {
							"http" => {
								break 'proto cli_core::rivet_cloud::model::ProxyProtocol::Http;
							}
							"https" => {
								break 'proto cli_core::rivet_cloud::model::ProxyProtocol::Https;
							}
							_ => {
								term::status::error("Invalid protocol", "");
								eprintln!();
							}
						}
					};

					let label = term::input::string_with_tip(
						term,
						"Development port label?",
						"example: default",
					)
					.await?;

					lobby_ports.push(
						cli_core::rivet_cloud::model::LobbyGroupRuntimeDockerPort::builder()
							.label(&label)
							.target_port(port as i32)
							.proxy_protocol(proto.clone())
							.build(),
					);

					term::status::success(
						format!("Added {label}"),
						format!("port={port} proto={proto:?}"),
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
					.set_lobby_ports(Some(lobby_ports))
					.send()
					.await
					.context("client.create_game_namespace_token_development")?;
				let token = token_res.token().context("token_res.token")?;

				eprintln!();
				term::status::success(format!("Token created"), "");

				eprintln!();
				if term::input::bool(term, "Write token to .env file?").await? {
					let env_file =
						format!("RIVET_CLIENT_TOKEN={token}\nRIVET_LOBBY_TOKEN={token}\n");
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
