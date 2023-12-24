use clap::Parser;
use cli_core::rivet_api::{apis, models};
use global_error::prelude::*;
use serde::Serialize;
use serde_json::json;
use uuid::Uuid;

use crate::{
	commands::{cdn, config, docker, version},
	util::{struct_fmt, term},
};

#[derive(Parser)]
pub struct Opts {
	/// Name of the version to create
	#[clap(long = "name", alias = "display-name")]
	display_name: Option<String>,

	/// Override specific properties of the config
	#[clap(long = "override", short)]
	overrides: Vec<String>,

	/// Namespace ID to deploy to
	#[clap(short = 'n', long)]
	namespace: Option<String>,

	/// Number of files to upload in parallel
	#[clap(long, env = "RIVET_CONCURRENT_UPLOADS", default_value = "8")]
	concurrent_uploads: usize,

	#[clap(long, value_parser)]
	format: Option<struct_fmt::Format>,

	/// Deprecated.
	///
	/// The build tag to upload
	#[clap(hide = true, long)]
	build_tag: Option<String>,

	/// Deprecated.
	///
	/// The name to assign to the build
	#[clap(hide = true, long)]
	build_name: Option<String>,

	/// Deprecated.
	///
	/// The path to the site directory to upload
	#[clap(hide = true, long)]
	site_path: Option<String>,

	/// Deprecated.
	///
	/// The name of the site that will be created
	#[clap(hide = true, long)]
	site_name: Option<String>,
}

impl Opts {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> GlobalResult<()> {
		// Validate config
		let errors = config::ValidateOpts {
			overrides: self.overrides.clone(),
			namespace: self.namespace.clone(),
			print: false,
		}
		.execute(ctx)
		.await?;
		if !errors.is_empty() {
			eprintln!("Found errors:");
			for error in errors {
				println!("- {error:?}");
			}

			bail!("config is invalid")
		}

		// Parse args
		let mut overrides = config::parse_config_override_args(&self.overrides)?;

		// Build & push site & build before creating version
		build_and_push_compat(
			ctx,
			&mut overrides,
			&self.build_tag,
			&self.build_name,
			&self.site_path,
			&self.site_name,
			self.concurrent_uploads,
			&self.format,
		)
		.await?;

		// Create version
		let output = deploy(
			ctx,
			self.display_name.as_ref().map(String::as_str),
			overrides,
			self.namespace.as_ref().map(String::as_str),
			self.concurrent_uploads,
			self.format.as_ref(),
		)
		.await?;
		struct_fmt::print_opt(self.format.as_ref(), &output)?;

		Ok(())
	}
}

/// Backwards compatibility for site & Docker build pushing from the CLI flags.
///
/// Developers should override config properties instead. For example: `rivet deploy -o matchmaker.docker.image_id=xxxx -o
/// cdn.path=xxxx`
pub async fn build_and_push_compat(
	ctx: &cli_core::Ctx,
	overrides: &mut Vec<(String, serde_json::Value)>,
	build_tag: &Option<String>,
	build_name: &Option<String>,
	site_path: &Option<String>,
	site_name: &Option<String>,
	concurrent_uploads: usize,
	format: &Option<struct_fmt::Format>,
) -> GlobalResult<()> {
	let site_output = if let Some(site_path) = site_path {
		Some(
			cdn::push(
				ctx,
				&cdn::PushOpts {
					path: site_path.clone(),
					name: site_name.clone(),
					concurrent_uploads,
					format: format.clone(),
				},
			)
			.await?,
		)
	} else {
		None
	};

	let build_output = if let Some(build_tag) = build_tag {
		Some(
			docker::push(
				ctx,
				&docker::PushOpts {
					tag: build_tag.clone(),
					name: build_name.clone(),
					format: format.clone(),
				},
			)
			.await?,
		)
	} else {
		None
	};

	if let Some(site_output) = site_output {
		overrides.push(("cdn.site_id".into(), json!(site_output.site_id)));
	}
	if let Some(build_output) = build_output {
		overrides.push((
			"matchmaker.docker.image_id".into(),
			json!(build_output.image_id),
		));
	}

	Ok(())
}

#[derive(Serialize)]
pub struct DeployOutput {
	pub version_id: Uuid,
}

/// Deploys a new Rivet version.
pub async fn deploy(
	ctx: &cli_core::Ctx,
	display_name: Option<&str>,
	overrides: Vec<(String, serde_json::Value)>,
	namespace_name_id: Option<&str>,
	concurrent_uploads: usize,
	format: Option<&struct_fmt::Format>,
) -> GlobalResult<DeployOutput> {
	// Fetch game data
	let game_res = apis::cloud_games_games_api::cloud_games_games_get_game_by_id(
		&ctx.openapi_config_cloud,
		&ctx.game_id,
		None,
	)
	.await?;
	let namespace = if let Some(namespace) = namespace_name_id {
		Some(unwrap!(game_res
			.game
			.namespaces
			.iter()
			.find(|x| x.name_id == namespace)))
	} else {
		None
	};

	// Choose display name
	let display_name = if let Some(x) = &display_name {
		// Use predefined version

		x.to_string()
	} else {
		// Generate version name

		let reserve_res =
			apis::cloud_games_versions_api::cloud_games_versions_reserve_version_name(
				&ctx.openapi_config_cloud,
				&ctx.game_id,
			)
			.await?;
		reserve_res.version_display_name
	};
	term::status::info("Deploying", &display_name);

	// Parse config
	let mut rivet_config = config::read_config(overrides, namespace_name_id).await?;
	build_config_dependencies(
		ctx,
		&mut rivet_config,
		&display_name,
		concurrent_uploads,
		format,
	)
	.await?;

	// Create game version
	let has_cdn = rivet_config.cdn.is_some();
	let version_res = apis::cloud_games_versions_api::cloud_games_versions_create_game_version(
		&ctx.openapi_config_cloud,
		&ctx.game_id,
		models::CloudGamesCreateGameVersionRequest {
			display_name: display_name.clone(),
			config: Box::new(rivet_config),
		},
	)
	.await;
	if let Err(err) = version_res.as_ref() {
		println!("Error: {err:?}");
	}
	let version_res = unwrap!(version_res);
	let version_id = version_res.version_id;

	eprintln!();
	term::status::success("Deployed Version", &display_name);
	term::status::info(
		"Version Dashboard",
		version::dashboard_url(&ctx.game_id, &version_id.to_string()),
	);

	// Deploy to namespace
	if let Some(namespace) = namespace {
		eprintln!();
		term::status::info(
			"Deploying to Namespace",
			format!("{} -> {}", display_name, namespace.display_name),
		);
		let update_version_res =
			apis::cloud_games_namespaces_api::cloud_games_namespaces_update_game_namespace_version(
				&ctx.openapi_config_cloud,
				&ctx.game_id,
				&namespace.namespace_id.to_string(),
				models::CloudGamesNamespacesUpdateGameNamespaceVersionRequest {
					version_id: version_id.clone(),
				},
			)
			.await;
		if let Err(err) = update_version_res.as_ref() {
			println!("Error: {err:?}");
		}
		unwrap!(update_version_res);
		if let (true, Some(domains)) = (has_cdn, &ctx.bootstrap.domains) {
			term::status::success(
				"Deploy Succeeded",
				version::rivet_game_url(&domains.cdn, &game_res.game.name_id, &namespace.name_id),
			);
		} else {
			term::status::success("Deploy Succeeded", "");
		}
	}

	eprintln!();

	Ok(DeployOutput { version_id })
}

/// Builds the Docker image and CDN site if needed.
pub async fn build_config_dependencies(
	ctx: &cli_core::Ctx,
	version: &mut models::CloudVersionConfig,
	display_name: &str,
	concurrent_uploads: usize,

	format: Option<&struct_fmt::Format>,
) -> GlobalResult<()> {
	if let Some(matchmaker) = version.matchmaker.as_mut() {
		// matchmaker.docker
		let default_image_id = if let Some(docker) = matchmaker.docker.as_mut() {
			let image_id = build_and_push_image(ctx, display_name, docker, format, None).await?;
			docker.image_id = image_id;
			image_id
		} else {
			None
		};

		// matchmaker.game_modes.*.docker
		if let Some(game_modes) = matchmaker.game_modes.as_mut() {
			for (_, game_mode) in game_modes.iter_mut() {
				if let Some(docker) = game_mode.docker.as_mut() {
					docker.image_id =
						build_and_push_image(ctx, display_name, docker, format, default_image_id)
							.await?;
				}
			}
		}
	}

	// Build CDN
	if let Some(cdn) = version.cdn.as_mut() {
		build_and_push_site(ctx, display_name, cdn, concurrent_uploads, format).await?;
	}

	Ok(())
}

/// Builds image if not specified and returns the image ID.
///
/// The image ID is chosen in order of priority:
///
/// - `dockerfile` Build the Dockerfile
/// - `image` Upload a prebuilt image
/// - `default_image_id` Use the image ID defined at the base of the matchmaker config
///
/// If none are true, `None` is returned.
pub async fn build_and_push_image(
	ctx: &cli_core::Ctx,
	display_name: &str,
	docker: &mut Box<models::CloudVersionMatchmakerGameModeRuntimeDocker>,
	format: Option<&struct_fmt::Format>,
	default_image_id: Option<Uuid>,
) -> GlobalResult<Option<Uuid>> {
	if docker.image_id.is_none() {
		if let Some(dockerfile) = &docker.dockerfile {
			let push_output = docker::build_and_push(
				ctx,
				&docker::BuildPushOpts {
					dockerfile: dockerfile.clone(),
					name: Some(display_name.to_string()),
					format: format.cloned(),
				},
			)
			.await?;

			return Ok(Some(push_output.image_id));
		} else if let Some(docker_image) = docker.image.as_ref() {
			let push_output = docker::push(
				ctx,
				&docker::PushOpts {
					tag: docker_image.clone(),
					name: Some(display_name.to_string()),
					format: format.cloned(),
				},
			)
			.await?;

			return Ok(Some(push_output.image_id));
		} else if let Some(image_id) = default_image_id {
			return Ok(Some(image_id));
		}
	}

	Ok(None)
}
pub async fn build_and_push_site(
	ctx: &cli_core::Ctx,
	display_name: &str,
	cdn: &mut Box<models::CloudVersionCdnConfig>,
	concurrent_uploads: usize,
	format: Option<&struct_fmt::Format>,
) -> GlobalResult<()> {
	if cdn.site_id.is_none() {
		if let Some(build_output) = &cdn.build_output {
			if let Some(build_command) = &cdn.build_command {
				let push_output = cdn::build_and_push(
					ctx,
					&cdn::BuildPushOpts {
						command: build_command.clone(),
						path: build_output.clone(),
						name: Some(display_name.to_string()),
						concurrent_uploads,
						format: format.cloned(),
					},
				)
				.await?;
				cdn.site_id = Some(push_output.site_id);
			} else {
				let push_output = cdn::push(
					ctx,
					&cdn::PushOpts {
						path: build_output.clone(),
						name: Some(display_name.to_string()),
						concurrent_uploads,
						format: format.cloned(),
					},
				)
				.await?;
				cdn.site_id = Some(push_output.site_id);
			}
		}
	}

	Ok(())
}
