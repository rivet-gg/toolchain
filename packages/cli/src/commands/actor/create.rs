use anyhow::*;
use clap::{Parser, ValueEnum};
use serde::Deserialize;
use std::{collections::HashMap, process::ExitCode};
use toolchain::rivet_api::{apis, models};
use uuid::Uuid;

use crate::util::kv_str;

#[derive(ValueEnum, Clone)]
enum NetworkMode {
	Bridge,
	Host,
}

/// Custom struct that includes the port name in it. The name is mapped to the key in the `ports`
/// map.
#[derive(Deserialize)]
struct Port {
	name: String,
	protocol: models::ActorPortProtocol,
	internal_port: Option<i32>,
	guard: Option<models::ActorGuardRouting>,
	#[serde(default)]
	host: bool,
}

#[derive(Parser)]
pub struct Opts {
	#[clap(index = 1)]
	environment: String,

	#[clap(long, short = 'r')]
	region: String,

	#[clap(long, short = 't')]
	tags: Option<String>,

	#[clap(long)]
	build: Option<String>,

	#[clap(long, short = 'b')]
	build_tags: Option<String>,

	#[clap(long = "arg")]
	arguments: Option<Vec<String>>,

	#[clap(long = "env")]
	env_vars: Option<Vec<String>>,

	#[clap(long, value_enum)]
	network_mode: Option<NetworkMode>,

	#[clap(long = "port", short = 'p')]
	ports: Option<Vec<String>>,

	#[clap(long, default_value = "1000")]
	cpu: i32,

	#[clap(long, default_value = "1024")]
	memory: i32,

	#[clap(long)]
	kill_timeout: Option<i64>,

	#[clap(long)]
	durable: bool,

	#[clap(long)]
	logs: bool,

	#[clap(long)]
	log_stream: Option<crate::util::actor::logs::LogStream>,
}

impl Opts {
	pub async fn execute(&self) -> ExitCode {
		match self.execute_inner().await {
			Result::Ok(code) => code,
			Err(err) => {
				eprintln!("{err}");
				ExitCode::FAILURE
			}
		}
	}

	pub async fn execute_inner(&self) -> Result<ExitCode> {
		let ctx = toolchain::toolchain_ctx::load().await?;

		// Parse tags
		let tags = self
			.tags
			.as_ref()
			.map(|tags_str| kv_str::from_str::<HashMap<String, String>>(tags_str))
			.transpose()?
			.unwrap_or_else(|| HashMap::new());

		// Parse build tags
		let build_tags = self
			.build_tags
			.as_ref()
			.map(|tags_str| kv_str::from_str::<HashMap<String, String>>(tags_str))
			.transpose()?;

		// Parse ports
		let ports = self
			.ports
			.as_ref()
			.map(|ports| {
				ports
					.iter()
					.map(|port_str| {
						let port = kv_str::from_str::<Port>(port_str)?;
						Ok((
							port.name,
							models::ActorCreateActorPortRequest {
								internal_port: port.internal_port,
								protocol: port.protocol,
								routing: Some(Box::new(models::ActorPortRouting {
									guard: port.guard.map(Box::new),
									host: if port.host {
										Some(serde_json::json!({}))
									} else {
										None
									},
								})),
							},
						))
					})
					.collect::<Result<HashMap<String, models::ActorCreateActorPortRequest>>>()
			})
			.transpose()?;

		// Parse environment variables
		let env_vars = self
			.env_vars
			.as_ref()
			.map(|env_vars| {
				env_vars
					.iter()
					.map(|env| {
						env.split_once('=')
							.map(|(k, v)| (k.to_string(), v.to_string()))
							.with_context(|| anyhow!("invalid env value: {env}"))
					})
					.collect::<Result<HashMap<String, String>>>()
			})
			.transpose()?;

		let request = models::ActorCreateActorRequest {
			region: self.region.clone(),
			tags: Some(serde_json::json!(tags)),
			build: self
				.build
				.as_ref()
				.map(|b| Uuid::parse_str(&b))
				.transpose()
				.context("invalid build uuid")?,
			build_tags: build_tags.map(|bt| Some(serde_json::json!(bt))),
			runtime: Box::new(models::ActorCreateActorRuntimeRequest {
				arguments: self.arguments.clone(),
				environment: env_vars,
			}),
			network: Some(Box::new(models::ActorCreateActorNetworkRequest {
				mode: self.network_mode.as_ref().map(|mode| match mode {
					NetworkMode::Bridge => models::ActorNetworkMode::Bridge,
					NetworkMode::Host => models::ActorNetworkMode::Host,
				}),
				ports,
			})),
			resources: Box::new(models::ActorResources {
				cpu: self.cpu,
				memory: self.memory,
			}),
			lifecycle: Some(Box::new(models::ActorLifecycle {
				durable: Some(self.durable),
				kill_timeout: self.kill_timeout,
			})),
		};

		let actor_id = match apis::actor_api::actor_create(
			&ctx.openapi_config_cloud,
			request,
			Some(&ctx.project.name_id),
			Some(&self.environment),
		)
		.await
		{
			Result::Ok(response) => {
				println!("Created actor:\n{:#?}", response.actor);
				response.actor.id
			}
			Err(e) => {
				eprintln!("Failed to create actor: {}", e);
				return Ok(ExitCode::FAILURE);
			}
		};

		// Tail logs
		if self.logs {
			crate::util::actor::logs::tail(
				&ctx,
				crate::util::actor::logs::TailOpts {
					environment: &self.environment,
					actor_id,
					stream: self
						.log_stream
						.clone()
						.unwrap_or(crate::util::actor::logs::LogStream::StdOut),
					follow: true,
					timestamps: true,
				},
			)
			.await?;
		}

		Ok(ExitCode::SUCCESS)
	}
}
