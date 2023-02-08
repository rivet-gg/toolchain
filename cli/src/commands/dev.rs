use anyhow::{bail, Context, Result};
use clap::Parser;
use cli_core::rivet_api::models;
use console::Term;
use serde::Serialize;
use std::collections::HashMap;
use tokio::fs;

use crate::{
	commands,
	util::{struct_fmt, term},
};

#[derive(Parser)]
pub enum SubCommand {
	CreateDevToken(CreateDevTokenOpts),
}

impl SubCommand {
	pub async fn execute(&self, term: &Term, ctx: &cli_core::Ctx) -> Result<()> {
		match self {
			SubCommand::CreateDevToken(opts) => opts.execute(term, ctx).await,
		}
	}
}

#[derive(Parser)]
pub struct CreateDevTokenOpts {
	/// Write token to .env file
	#[clap(long)]
	dev_env: bool,
	#[clap(long, value_parser)]
	format: Option<struct_fmt::Format>,
}

impl CreateDevTokenOpts {
	pub async fn execute(&self, term: &Term, ctx: &cli_core::Ctx) -> Result<()> {
		let output = create_dev_token(term, ctx, self).await?;
		struct_fmt::print_opt(self.format.as_ref(), &output)?;

		Ok(())
	}
}

#[derive(Serialize)]
pub struct CreateDevTokenOutput {
	pub token: String,
}

pub async fn create_dev_token(
	term: &Term,
	ctx: &cli_core::Ctx,
	opts: &CreateDevTokenOpts,
) -> Result<CreateDevTokenOutput> {
	let ns_name_id = "staging";

	let game_res = ctx
		.client()
		.get_game_by_id()
		.game_id(&ctx.game_id)
		.send()
		.await
		.context("client.get_game_by_id")?;
	let game = game_res.game.context("game_res.game")?;
	let game_ns = game.namespaces().context("game.namespaces")?;
	let staging_namespace_id = game_ns
		.iter()
		.find(|x| x.name_id().map_or(false, |x| x == ns_name_id))
		.and_then(|x| x.namespace_id())
		.context("game.namespaces.find(\"staging\").namespace_id")?;

	let config = commands::version::read_config(Vec::new(), Some(ns_name_id)).await?;

	let Some(matchmaker) = &config.matchmaker else {
			bail!("matchmaker not enabled")
		};

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

	let mut default_port = None;
	let dev_ports = dev_ports
		.into_iter()
		.map(|(label, port_config)| {
			let port = port_config
				.dev_port
				.or(port_config.port)
				.context(format!("missing both dev_port and port from {label}"))?;

			if default_port.is_none() {
				default_port = Some(port);
			}

			Ok((
				label,
				models::CloudMatchmakerDevelopmentPort {
					port,
					protocol: port_config
						.dev_protocol
						.or(port_config.protocol)
						.unwrap_or(models::CloudVersionMatchmakerProxyProtocol::Http),
				},
			))
		})
		.collect::<Result<HashMap<_, _>>>()?;

	// Create dev token
	let token_res = cli_core::rivet_api::apis::cloud_games_namespaces_api::cloud_games_namespaces_create_game_namespace_token_development(
			&ctx.openapi_config_cloud,
			&ctx.game_id,
			&staging_namespace_id,
			cli_core::rivet_api::models::CloudGamesNamespacesCreateGameNamespaceTokenDevelopmentInput {
				hostname: dev_hostname,
				ports: Some(dev_ports),
				lobby_ports: None,
			},
		)
		.await;
	if let Err(err) = token_res.as_ref() {
		println!("Error: {err:?}");
	}
	let token_res =
		token_res.context("cloud_games_namespaces_create_game_namespace_token_development")?;
	let token = token_res.token;

	eprintln!();
	term::status::success(format!("Token created"), "");

	eprintln!();
	if opts.dev_env
		|| term::Prompt::new("Write development token to .env file?")
			.docs("We recommend storing your token in a .env file to keep it secure")
			.docs_url("https://github.com/motdotla/dotenv#dotenv")
			.default_value("yes")
			.bool(term)
			.await?
	{
		let env_file = format!(
			"PORT={port}\nRIVET_TOKEN={token}\n",
			port = default_port.unwrap()
		);
		fs::write(".env", env_file).await?;
		term::status::success(format!("Wrote to .env"), "");
	}

	Ok(CreateDevTokenOutput { token })
}
