use anyhow::{ensure, Context, Result};
use clap::Parser;
use console::Term;
use tokio::fs;

use crate::util::term;

#[derive(Parser)]
pub enum SubCommand {
	Init(InitOpts),
}

#[derive(Parser)]
pub struct InitOpts {
	/// Local hostname to connect to
	#[clap(long)]
	dev_hostname: Option<String>,
	/// Local port to connect to
	#[clap(long)]
	dev_port: Option<Vec<String>>,
	/// Write token to .env file
	#[clap(long)]
	dev_env: bool,
}

impl SubCommand {
	pub async fn execute(&self, term: &Term, ctx: &cli_core::Ctx) -> Result<()> {
		match self {
			SubCommand::Init(opts) => opts.execute(term, ctx).await,
		}
	}
}

impl InitOpts {
	pub async fn execute(&self, term: &Term, ctx: &cli_core::Ctx) -> Result<()> {
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
		let hostname = if let Some(x) = self.dev_hostname.clone() {
			x
		} else {
			term::input::string_with_tip(term, "Dev hostname?", "example: 127.0.0.1").await?
		};

		let mut default_port = Option::<u16>::None;
		let mut lobby_ports = Vec::new();
		if let Some(dev_ports) = &self.dev_port {
			for port in dev_ports {
				let port_components = port.split(":").collect::<Vec<_>>();
				ensure!(
					port_components.len() == 3,
					"port must have 3 components: `{{label}}:{{proto}}:{{port}}`"
				);
				let label = port_components[0];
				let proto =
					parse_proxy_proto(port_components[1]).context("inalid proxy protocol")?;
				let port = port_components[2]
					.parse::<u16>()
					.context("port is not valid number")?;

				if default_port.is_none() {
					default_port = Some(port);
				}
				lobby_ports.push(
					cli_core::rivet_cloud::model::LobbyGroupRuntimeDockerPort::builder()
						.label(label)
						.target_port(port as i32)
						.proxy_protocol(proto.clone())
						.build(),
				);
			}
		} else {
			loop {
				eprintln!();

				let port = 'port: loop {
					let port =
						term::input::string_with_tip(term, "Dev port?", "0-65535, example: 8080")
							.await?;
					if let Ok(port) = port.parse::<u16>() {
						break 'port port;
					} else {
						term::status::error("Invalid port number", "");
						eprintln!();
					}
				};
				if default_port.is_none() {
					default_port = Some(port);
				}

				let proto = 'proto: loop {
					let proto = term::input::string_with_tip(
						term,
						"Dev port protocol?",
						"http/https, usually: http",
					)
					.await?;
					if let Some(proto) = parse_proxy_proto(&proto) {
						break 'proto proto;
					} else {
						term::status::error("Invalid protocol", "");
						eprintln!();
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
		if self.dev_env || term::input::bool(term, "Write token to .env file?").await? {
			let env_file = format!(
				"PORT={port}\nRIVET_TOKEN={token}\n",
				port = default_port.unwrap()
			);
			fs::write(".env", env_file).await?;
			term::status::success(format!("Wrote to .env"), "");
		} else {
			println!("{token}");
		}

		Ok(())
	}
}

fn parse_proxy_proto(proto: &str) -> Option<cli_core::rivet_cloud::model::ProxyProtocol> {
	match proto.to_lowercase().as_str() {
		"http" => Some(cli_core::rivet_cloud::model::ProxyProtocol::Http),
		"https" => Some(cli_core::rivet_cloud::model::ProxyProtocol::Https),
		_ => None,
	}
}
