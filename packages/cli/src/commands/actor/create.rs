use anyhow::*;
use clap::{Parser, ValueEnum};
use serde::Deserialize;
use std::{collections::HashMap, process::ExitCode};
use toolchain::{
	build,
	rivet_api::{apis, models},
};
use uuid::Uuid;

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
	region: Option<String>,

	/// Tags to use for both the actor & build tags. This allows for creating actors quickly since
	/// the tags are often identical between the two.
	#[clap(long = "tags", short = 't')]
	universal_tags: Option<String>,

	#[clap(long, short = 'a')]
	actor_tags: Option<String>,

	/// Build ID.
	#[clap(long)]
	build: Option<String>,

	#[clap(long, short = 'b')]
	build_tags: Option<String>,

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

	/// If included, the `current` tag will not be automatically inserted to the build tag.
	#[clap(long)]
	no_build_current_tag: bool,

	#[clap(long)]
	logs: bool,

	#[clap(long)]
	log_stream: Option<crate::util::actor::logs::LogStream>,

	#[clap(long)]
	deploy: bool,
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
		let actor_tags = if let Some(t) = &self.actor_tags {
			kv_str::from_str::<HashMap<String, String>>(t)?
		} else if let Some(t) = &self.universal_tags {
			kv_str::from_str::<HashMap<String, String>>(t)?
		} else {
			// No tags
			HashMap::new()
		};

		// Parse build ID
		let mut build_id = self
			.build
			.as_ref()
			.map(|b| Uuid::parse_str(&b))
			.transpose()
			.context("invalid build uuid")?;

		// Parse build tags
		let mut build_tags = if let Some(t) = &self.build_tags {
			Some(kv_str::from_str::<HashMap<String, String>>(t)?)
		} else if let Some(t) = &self.universal_tags {
			Some(kv_str::from_str::<HashMap<String, String>>(t)?)
		} else {
			None
		};

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

		// Auto-deploy
		if self.deploy {
			// Remove build tags, since we'll be using the build ID
			let build_tags = build_tags
				.take()
				.context("must define build tags when using deploy flag")?;

			// Deploys erver
			match crate::util::deploy::deploy(crate::util::deploy::DeployOpts {
				environment: &self.environment,
				build_tags: Some(build_tags),
			})
			.await
			{
				Result::Ok(deploy_build_ids) => {
					if deploy_build_ids.len() > 1 {
						println!("Warning: Multiple build IDs match tags, proceeding with first");
					}

					let deploy_build_id = deploy_build_ids
						.first()
						.context("No builds matched build tags")?;
					build_id = Some(*deploy_build_id);
				}
				Err(code) => {
					return Ok(code);
				}
			};
		}

		// Automatically add `current` tag to make querying easier
		//
		// Do this AFTER the deploy since this will mess up the build filter.
		if !self.no_build_current_tag {
			if let Some(build_tags) = build_tags.as_mut() {
				if !build_tags.contains_key(build::tags::VERSION) {
					build_tags.insert(build::tags::CURRENT.into(), "true".into());
				}
			}
		}

		// Auto-select region if needed
		let region = if let Some(region) = &self.region {
			region.clone()
		} else {
			let regions = apis::actor_regions_api::actor_regions_list(
				&ctx.openapi_config_cloud,
				Some(&ctx.project.name_id.to_string()),
				Some(&self.environment),
			)
			.await?;

			// TODO(RVT-4207): Improve automatic region selection logic
			// Choose a region
			let auto_region = if let Some(ideal_region) = regions
				.regions
				.iter()
				.filter(|r| r.id == "lax" || r.id == "local")
				.next()
			{
				ideal_region.id.clone()
			} else {
				regions.regions.first().context("no regions")?.id.clone()
			};
			println!("Automatically selected region: {auto_region}");

			auto_region
		};

		let request = models::ActorCreateActorRequest {
			region,
			tags: Some(serde_json::json!(actor_tags)),
			build: build_id,
			build_tags: build_tags.map(|bt| Some(serde_json::json!(bt))),
			runtime: Box::new(models::ActorCreateActorRuntimeRequest {
				arguments: None,
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
