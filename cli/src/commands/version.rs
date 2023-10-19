use anyhow::{bail, ensure, Context, Error, Result};
use clap::Parser;
use cli_core::rivet_api::models;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::str::FromStr;
use tabled::Tabled;
use tempfile::TempDir;
use tokio::process::Command;
use uuid::Uuid;

use crate::{
	commands::{image, site},
	util::{cmd, fmt, gen, lz4, struct_fmt, term},
};

/// Defines how Docker Build will be ran.
#[derive(strum::EnumString)]
enum DockerBuildMethod {
	#[strum(serialize = "buildx")]
	Buildx,

	#[strum(serialize = "native")]
	Native,
}

/// Defines what type of build to create.
#[derive(strum::EnumString)]
enum BuildKind {
	#[strum(serialize = "docker-image")]
	DockerImage,

	#[strum(serialize = "oci-bundle")]
	OciBundle,
}

/// Defines how to compress the output.
#[derive(strum::EnumString)]
enum BuildCompression {
	#[strum(serialize = "none")]
	None,

	#[strum(serialize = "lz4")]
	LZ4,
}

impl DockerBuildMethod {
	async fn auto() -> Result<Self> {
		// Determine build method from env
		if let Some(method) = std::env::var("_RIVET_DOCKER_BUILD_METHOD")
			.ok()
			.and_then(|x| DockerBuildMethod::from_str(&x).ok())
		{
			Ok(method)
		} else {
			// Validate that Buildx is installed
			let mut buildx_version_cmd = Command::new("docker");
			buildx_version_cmd.args(&["buildx", "version"]);
			let buildx_version =
				cmd::execute_docker_cmd_silent_failable(buildx_version_cmd).await?;

			if buildx_version.status.success() {
				Ok(DockerBuildMethod::Buildx)
			} else {
				println!("Docker Buildx not installed. Falling back to native build method.\n\nPlease install Buildx here: https://github.com/docker/buildx#installing");
				Ok(DockerBuildMethod::Native)
			}
		}
	}
}

impl BuildKind {
	async fn auto() -> Result<BuildKind> {
		// Determine build method from env
		if let Some(method) = std::env::var("_RIVET_BUILD_KIND")
			.ok()
			.and_then(|x| BuildKind::from_str(&x).ok())
		{
			Ok(method)
		} else {
			Ok(BuildKind::OciBundle)
		}
	}
}

impl BuildCompression {
	async fn auto(kind: &BuildKind) -> Result<BuildCompression> {
		// Determine build method from env
		if let Some(method) = std::env::var("_RIVET_BUILD_COMPRESSION")
			.ok()
			.and_then(|x| BuildCompression::from_str(&x).ok())
		{
			Ok(method)
		} else {
			Ok(match kind {
				BuildKind::DockerImage => BuildCompression::None,
				BuildKind::OciBundle => BuildCompression::LZ4,
			})
		}
	}
}

impl Default for DockerBuildMethod {
	fn default() -> Self {
		DockerBuildMethod::Buildx
	}
}

#[derive(Parser)]
pub enum SubCommand {
	/// List all versions
	List,

	/// Get details about a version
	Get {
		/// The version ID
		version: String,
	},

	/// Pushes the build and site and creates a new version
	#[clap(alias = "push-and-create", alias = "create", alias = "publish")]
	Deploy(DeployOpts),

	/// Returns the config for a version
	#[clap(alias = "read-config")]
	ValidateConfig {
		/// Override specific properties of the config
		#[clap(long = "override", short)]
		overrides: Vec<String>,

		/// The namespace ID to deploy to
		#[clap(short = 'n', long)]
		namespace: Option<String>,
	},

	/// Show the a version's dashboard
	#[clap(alias("dash"))]
	Dashboard { version: String },
}

impl SubCommand {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> Result<()> {
		match self {
			SubCommand::List => {
				let game_res = ctx
					.client()
					.get_game_by_id()
					.game_id(&ctx.game_id)
					.send()
					.await
					.context("client.get_game_by_id")?;
				let game = game_res.game.context("game_res.game")?;
				let namespaces = game.namespaces().context("game.namespaces")?;

				#[derive(Tabled)]
				struct Version {
					#[tabled(rename = "Name")]
					display_name: String,
					#[tabled(rename = "Namespaces")]
					namespaces: String,
					#[tabled(rename = "Created")]
					created: String,
					#[tabled(rename = "ID")]
					version_id: String,
				}

				let mut version = game
					.versions()
					.context("game.versions")?
					.iter()
					.map(|version| {
						let ns = namespaces
							.iter()
							.filter(|ns| ns.version_id() == version.version_id())
							.filter_map(|ns| ns.display_name())
							.collect::<Vec<&str>>()
							.join(", ");

						Ok(Version {
							display_name: version
								.display_name()
								.context("version.display_name")?
								.to_string(),
							namespaces: ns,
							created: fmt::date(version.create_ts().context("version.create_ts")?),
							version_id: version
								.version_id()
								.context("version.version_id")?
								.to_string(),
						})
					})
					.collect::<Result<Vec<_>>>()?;
				version.reverse();
				term::table(&version);

				Ok(())
			}
			SubCommand::Get { version } => {
				print_version(ctx, &version).await?;

				Ok(())
			}
			SubCommand::Deploy(opts) => opts.execute(ctx).await,
			SubCommand::ValidateConfig {
				overrides,
				namespace,
			} => {
				let overrides = parse_config_override_args(overrides)?;
				let mut rivet_config =
					read_config(overrides, namespace.as_ref().map(String::as_str)).await?;
				eprintln!("{:#?}", rivet_config);
				build_mock_config_dependencies(&mut rivet_config)?;

				// Validate game version
				let validate_res =
				cli_core::rivet_api::apis::cloud_games_versions_api::cloud_games_versions_validate_game_version(
					&ctx.openapi_config_cloud,
					&ctx.game_id,
					cli_core::rivet_api::models::CloudGamesValidateGameVersionRequest {
						display_name: "Mock Dispaly Name".into(),
						config: Box::new(rivet_config),
					},
				)
				.await;
				eprintln!();
				if let Err(err) = validate_res.as_ref() {
					eprintln!("Error: {err:?}");
				}
				let validate_res =
					validate_res.context("cloud_games_versions_validate_game_version")?;
				if !validate_res.errors.is_empty() {
					eprintln!("Found errors:");
					for error in validate_res.errors {
						println!("- {error:?}");
					}
				} else {
					eprintln!("Config is valid.");
				}

				Ok(())
			}
			SubCommand::Dashboard { version } => {
				// Check the version exists
				ctx.client()
					.get_game_version_by_id()
					.game_id(&ctx.game_id)
					.version_id(version)
					.send()
					.await
					.context("client.get_game_version_by_id")?;

				eprintln!("{}", term::link(dashboard_url(&ctx.game_id, version)));

				Ok(())
			}
		}
	}
}

#[derive(Parser)]
pub struct DeployOpts {
	/// Name of the version to create
	#[clap(long = "name", alias = "display-name")]
	display_name: Option<String>,

	/// Override specific properties of the config
	#[clap(long = "override", short)]
	overrides: Vec<String>,

	/// Namespace ID to deploy to
	#[clap(short = 'n', long)]
	namespace: Option<String>,

	/// Deprecated.
	///
	/// The build tag to upload
	#[clap(long)]
	build_tag: Option<String>,

	/// Deprecated.
	///
	/// The name to assign to the build
	#[clap(long)]
	build_name: Option<String>,

	/// Deprecated.
	///
	/// The path to the site directory to upload
	#[clap(long)]
	site_path: Option<String>,

	/// Deprecated.
	///
	/// The name of the site that will be created
	#[clap(long)]
	site_name: Option<String>,

	#[clap(long, value_parser)]
	format: Option<struct_fmt::Format>,
}

impl DeployOpts {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> Result<()> {
		// Parse overrides
		let mut overrides = parse_config_override_args(&self.overrides)?;

		build_and_push_compat(
			ctx,
			&mut overrides,
			&self.build_tag,
			&self.build_name,
			&self.site_path,
			&self.site_name,
			&self.format,
		)
		.await?;

		let output = create(
			ctx,
			self.display_name.as_ref().map(String::as_str),
			overrides,
			self.namespace.as_ref().map(String::as_str),
			self.format.as_ref(),
		)
		.await?;
		struct_fmt::print_opt(self.format.as_ref(), &output)?;

		Ok(())
	}
}

/// Prints information about a game version
async fn print_version(ctx: &cli_core::Ctx, version_id: &str) -> Result<()> {
	let version_res = ctx
		.client()
		.get_game_version_by_id()
		.game_id(&ctx.game_id)
		.version_id(version_id)
		.send()
		.await
		.context("client.get_game_version_by_id")?;
	let version = version_res.version().context("version_res.version")?;

	println!("{version:#?}");

	Ok(())
}

/// Parses config parameters passed to override version parameters
pub fn parse_config_override_args(
	overrides: &[String],
) -> Result<Vec<(String, serde_json::Value)>> {
	overrides
		.iter()
		.map(|value| {
			value
				.split_once("=")
				.context("override needs equal")
				.and_then(|(key, value)| {
					let value_json = serde_json::from_str::<serde_json::Value>(value)
						.context("invalid override value json")?;
					Ok((key.to_string(), value_json))
				})
		})
		.collect::<Result<Vec<_>, Error>>()
}

/// Reads the Rivet configuration file and applies overrides. Uses the
/// namespace to read override files.
///
/// For example, in the namespace `foobar`, Rivet would first read
/// `rivet.toml` then override with properties from
/// `rivet.foobar.toml`.
pub async fn read_config(
	overrides: Vec<(String, serde_json::Value)>,
	namespace: Option<&str>,
) -> Result<models::CloudVersionConfig> {
	// Build base config
	let mut config_builder = config::ConfigBuilder::<config::builder::AsyncState>::default()
		.add_source(config::File::with_name("rivet").required(false))
		// Support legacy `rivet.version.toml` file name
		.add_source(config::File::with_name("rivet.version").required(false));

	if let Some(namespace) = namespace {
		config_builder = config_builder
			.add_source(config::File::with_name(&format!("rivet.{namespace}")).required(false))
			.add_source(
				config::File::with_name(&format!("rivet.version.{namespace}")).required(false),
			);
	}

	// Apply overrides
	for (k, v) in overrides {
		#[derive(serde::Serialize)]
		struct Empty {
			root: serde_json::Value,
		}

		// Parse the JSON data to a config Value that we can pass as an
		// override.
		//
		// We have to embed the value in `Empty` because the value can't be at
		// the root of the config.
		let config_value = config::Config::try_from(&Empty { root: v })
			.context("read override value to config value")?
			.get::<config::Value>("root")?;

		// Add the override
		config_builder = config_builder
			.set_override(k, config_value)
			.context("set override")?;
	}

	// Read config
	let config = config_builder.build().await.context("find config")?;
	let version = config
		.try_deserialize::<models::CloudVersionConfig>()
		.context("deserialize config")?;

	Ok(version)
}

/// Fill in dummy information for fields that will be eventually filled in by the
/// build config dependencies.
///
/// Used to build a config file that wil pass validation.
pub fn build_mock_config_dependencies(version: &mut models::CloudVersionConfig) -> Result<()> {
	if let Some(matchmaker) = version.matchmaker.as_mut() {
		if let Some(docker) = matchmaker.docker.as_mut() {
			docker.image_id = Some(Uuid::nil());
		}

		if let Some(game_modes) = matchmaker.game_modes.as_mut() {
			for (_, game_mode) in game_modes.iter_mut() {
				if let Some(docker) = game_mode.docker.as_mut() {
					docker.image_id = Some(Uuid::nil());
				}
			}
		}
	}

	// Build CDN
	if let Some(cdn) = version.cdn.as_mut() {
		cdn.site_id = Some(Uuid::nil());
	}

	Ok(())
}

/// Builds the Docker image and CDN site if needed.
pub async fn build_config_dependencies(
	ctx: &cli_core::Ctx,
	version: &mut models::CloudVersionConfig,
	format: Option<&struct_fmt::Format>,
) -> Result<()> {
	// TODO: Do this for all possible docker endpoints

	if let Some(matchmaker) = version.matchmaker.as_mut() {
		if let Some(docker) = matchmaker.docker.as_mut() {
			build_image(ctx, docker, format).await?;
		}

		if let Some(game_modes) = matchmaker.game_modes.as_mut() {
			for (_, game_mode) in game_modes.iter_mut() {
				if let Some(docker) = game_mode.docker.as_mut() {
					build_image(ctx, docker, format).await?;
				}
			}
		}
	}

	// Build CDN
	if let Some(cdn) = version.cdn.as_mut() {
		build_site(ctx, cdn, format).await?;
	}

	Ok(())
}

pub async fn build_image(
	ctx: &cli_core::Ctx,
	docker: &mut Box<models::CloudVersionMatchmakerGameModeRuntimeDocker>,
	format: Option<&struct_fmt::Format>,
) -> Result<()> {
	if docker.image_id.is_none() {
		if let Some(dockerfile) = docker.dockerfile.as_ref() {
			let build_method = DockerBuildMethod::auto().await?;
			let build_kind = BuildKind::auto().await?;
			let build_compression = BuildCompression::auto(&build_kind).await?;

			eprintln!();
			let buildx_info = match build_method {
				DockerBuildMethod::Native => " (with native)",
				DockerBuildMethod::Buildx => " (with buildx)",
			};
			term::status::info("Building Image", format!("{dockerfile}{buildx_info}"));

			// Create temp path to write to
			let build_tar_file = tempfile::NamedTempFile::new()?;
			let build_tar_path = build_tar_file.into_temp_path();

			// Build image
			let unique_image_tag = format!("rivet-game:{}", Uuid::new_v4());
			match build_method {
				DockerBuildMethod::Native => {
					let mut build_cmd = Command::new("docker");
					build_cmd
						.arg("build")
						.arg("--file")
						.arg(dockerfile)
						.arg("--tag")
						.arg(&unique_image_tag)
						.arg(".");
					cmd::execute_docker_cmd(build_cmd, "Docker image failed to build").await?;
				}
				DockerBuildMethod::Buildx => {
					let builder_name = "rivet_cli";

					// Determine if needs to create a new builder
					let mut inspect_cmd = Command::new("docker");
					inspect_cmd.arg("buildx").arg("inspect").arg(builder_name);
					let inspect_output =
						cmd::execute_docker_cmd_silent_failable(inspect_cmd).await?;

					if !inspect_output.status.success()
						&& String::from_utf8(inspect_output.stderr.clone())?
							.contains(&format!("no builder \"{builder_name}\" found"))
					{
						// Create new builder
						let mut build_cmd = Command::new("docker");
						build_cmd
							.arg("buildx")
							.arg("create")
							.arg("--name")
							.arg(builder_name)
							.arg("--driver")
							.arg("docker-container")
							.arg("--platform")
							.arg("linux/amd64");
						cmd::execute_docker_cmd(
							build_cmd,
							"Failed to create Docker Buildx builder",
						)
						.await?;
					} else {
						// Builder exists

						cmd::error_for_output_failure(
							&inspect_output,
							"Failed to inspect Docker Buildx runner",
						)?;
					}

					// Build image
					let mut build_cmd = Command::new("docker");
					build_cmd
						.arg("buildx")
						.arg("build")
						.arg("--builder")
						.arg(builder_name)
						.arg("--platform")
						.arg("linux/amd64")
						.arg("--file")
						.arg(dockerfile)
						.arg("--tag")
						.arg(&unique_image_tag)
						.arg("--output")
						.arg("type=docker")
						.arg(".");
					cmd::execute_docker_cmd(build_cmd, "Docker image failed to build").await?;
				}
			}

			match build_kind {
				BuildKind::DockerImage => {
					// Save the Docker image to a TAR

					let mut build_cmd = Command::new("docker");
					build_cmd
						.arg("save")
						.arg("--output")
						.arg(&build_tar_path)
						.arg(&unique_image_tag);
					cmd::execute_docker_cmd(build_cmd, "Docker failed to save image").await?;
				}
				BuildKind::OciBundle => {
					// Convert the Docker image to an OCI bundle

					let bundle_dir = TempDir::new()?;

					let container_name = format!("rivet-game-{}", Uuid::new_v4());

					let mut create_cmd = Command::new("docker");
					create_cmd
						.arg("container")
						.arg("create")
						.arg("--name")
						.arg(&container_name)
						.arg(&unique_image_tag);
					cmd::execute_docker_cmd_silent(create_cmd, "Docker failed to create container")
						.await?;

					let mut cp_cmd = Command::new("docker");
					cp_cmd
						.arg("container")
						.arg("cp")
						.arg("--archive")
						.arg(format!("{container_name}:/"))
						.arg(bundle_dir.path().join("rootfs"));
					cmd::execute_docker_cmd_silent(
						cp_cmd,
						"Docker failed to copy files out of container",
					)
					.await?;

					let mut rm_cmd = Command::new("docker");
					rm_cmd
						.arg("container")
						.arg("rm")
						.arg("--force")
						.arg(&container_name);
					cmd::execute_docker_cmd_silent(rm_cmd, "Docker failed to remove container")
						.await?;

					let mut inspect_cmd = Command::new("docker");
					inspect_cmd
						.arg("image")
						.arg("inspect")
						.arg(&unique_image_tag);
					let inspect_output =
						cmd::execute_docker_cmd_silent_failable(inspect_cmd).await?;

					// Convert Docker image to OCI bundle
					//
					// See umoci implementation: https://github.com/opencontainers/umoci/blob/312b2db3028f823443d6a74d86b05f65701b0d0e/oci/config/convert/runtime.go#L183
					{
						#[derive(Deserialize)]
						#[serde(rename_all = "PascalCase")]
						struct DockerImage {
							config: DockerImageConfig,
						}

						#[derive(Deserialize)]
						#[serde(rename_all = "PascalCase")]
						struct DockerImageConfig {
							cmd: Option<Vec<String>>,
							entrypoint: Option<Vec<String>>,
							env: Vec<String>,
							user: String,
							#[serde(default)]
							working_dir: String,
						}

						// Parse image
						let image =
							serde_json::from_slice::<Vec<DockerImage>>(&inspect_output.stdout)?;
						let image = image.into_iter().next().context("no image")?;

						// Read config
						let mut config = serde_json::from_slice::<serde_json::Value>(
							include_bytes!("../../static/config.json"),
						)?;

						// WORKDIR
						//
						// https://github.com/opencontainers/umoci/blob/312b2db3028f823443d6a74d86b05f65701b0d0e/oci/config/convert/runtime.go#L144
						if image.config.working_dir != "" {
							config["process"]["cwd"] = json!(image.config.working_dir);
						} else {
							config["process"]["cwd"] = json!("/");
						}

						// ENV
						//
						// https://github.com/opencontainers/umoci/blob/312b2db3028f823443d6a74d86b05f65701b0d0e/oci/config/convert/runtime.go#L149
						config["process"]["env"] = json!(image.config.env);

						// ENTRYPOINT + CMD
						//
						// https://github.com/opencontainers/umoci/blob/312b2db3028f823443d6a74d86b05f65701b0d0e/oci/config/convert/runtime.go#L157
						let args = std::iter::empty::<String>()
							.chain(image.config.entrypoint.into_iter().flatten())
							.chain(image.config.cmd.into_iter().flatten())
							.collect::<Vec<_>>();
						config["process"]["args"] = json!(args);

						// USER
						//
						// https://github.com/opencontainers/umoci/blob/312b2db3028f823443d6a74d86b05f65701b0d0e/oci/config/convert/runtime.go#L174
						//
						// Moby passwd parser: https://github.com/moby/sys/blob/c0711cde08c8fa33857a2c28721659267f49b5e2/user/user.go
						//
						// If you're you're the guy at Docker who decided to reimplement passwd in Go for funzies, please reconsider next time.
						{
							// Parse user
							let (user, group) =
								if let Some((u, g)) = image.config.user.split_once(":") {
									(u, Some(g))
								} else {
									(image.config.user.as_str(), None)
								};

							// Attempt to parse user to uid
							let user_int = user.parse::<u32>().ok();
							let group_int = group.and_then(|x| x.parse::<u32>().ok());

							// Parse passwd file and find user
							let users = crate::util::users::read_passwd_file(
								&bundle_dir.path().join("rootfs/etc/passwd"),
							)?;
							let exec_user = users.iter().find(|x| {
								user_int.map_or(false, |uid| x.uid == uid) || x.name == user
							});

							// Determine uid
							let uid = if image.config.user.is_empty() {
								0
							} else if let Some(exec_user) = exec_user {
								exec_user.uid
							} else if let Some(uid) = user_int {
								uid
							} else {
								term::status::warn("Cannot determine uid", format!("{} not in passwd file, please specify a raw uid like `USER 1000:1000`", image.config.user));
								0
							};

							// Parse group file and find group
							let groups = crate::util::users::read_group_file(
								&bundle_dir.path().join("rootfs/etc/group"),
							)?;
							let exec_group = groups.iter().find(|x| {
								if let Some(group) = group {
									if let Some(gid) = group_int {
										return x.gid == gid;
									} else {
										x.name == group
									}
								} else if let Some(exec_user) = &exec_user {
									x.user_list.contains(&exec_user.name)
								} else {
									false
								}
							});

							// Determine gid
							let gid = if image.config.user.is_empty() {
								0
							} else if let Some(exec_group) = exec_group {
								exec_group.gid
							} else if let Some(gid) = group_int {
								gid
							} else {
								term::status::warn("Cannot determine gid", format!("{} not in group file, please specify a raw uid & gid like `USER 1000:1000`", image.config.user));

								0
							};

							// Validate not running as root
							//
							// See Kubernetes implementation https://github.com/kubernetes/kubernetes/blob/cea1d4e20b4a7886d8ff65f34c6d4f95efcb4742/pkg/kubelet/kuberuntime/security_context_others.go#L44C4-L44C4
							if std::env::var("_RIVET_OCI_BUNDLE_ALLOW_ROOT")
								.ok()
								.map_or(false, |x| &x == "1")
							{
								if uid == 0 {
									bail!("cannot run Docker container as root (i.e. uid 0) for security. see https://docs.docker.com/engine/reference/builder/#user")
								}
							}

							// Specify user
							config["process"]["user"]["uid"] = json!(uid);
							config["process"]["user"]["gid"] = json!(gid);

							// Add home if needed
							if let Some(home) = exec_user.as_ref().map(|x| x.home.as_str()) {
								if !home.is_empty() {
									config["process"]["env"]
										.as_array_mut()
										.unwrap()
										.push(json!(format!("HOME={home}")));
								}
							}
						}

						// Write config.json
						tokio::fs::write(
							bundle_dir.path().join("config.json"),
							serde_json::to_vec(&config)?,
						)
						.await?;
					}

					// Archive the bundle
					let mut archive_cmd = Command::new("tar");
					archive_cmd
						.arg("-cf")
						.arg(&build_tar_path)
						.arg(bundle_dir.path());
					let archive_status = archive_cmd.status().await?;
					ensure!(archive_status.success(), "failed to archive oci bundle");
				}
			}

			// Clean up image from the registry
			let mut remove_img_cmd = Command::new("docker");
			remove_img_cmd
				.arg("image")
				.arg("rm")
				.arg("--force")
				.arg(&unique_image_tag);
			cmd::execute_docker_cmd_silent_failable(remove_img_cmd).await?;

			// Compress the bundle
			let build_tar_compressed_file = tempfile::NamedTempFile::new()?;
			let build_tar_compressed_path = build_tar_compressed_file.into_temp_path();
			match build_compression {
				BuildCompression::None => {
					tokio::fs::rename(&build_tar_path, &build_tar_compressed_path).await?;
				}
				BuildCompression::LZ4 => {
					let build_tar_path = build_tar_path.to_owned();
					let build_tar_compressed_path = build_tar_compressed_path.to_owned();
					tokio::task::spawn_blocking(move || {
						lz4::compress(&build_tar_path, &build_tar_compressed_path)
					})
					.await??;
				}
			}

			// Upload build
			let push_output = image::push_tar(
				ctx,
				&image::ImagePushTarOpts {
					path: build_tar_path.to_owned(),
					tag: unique_image_tag,
					name: Some(gen::display_name_from_date()),
					format: format.cloned(),
				},
			)
			.await?;
			docker.image_id = Some(push_output.image_id);
		} else if let Some(docker_image) = docker.image.as_ref() {
			// Upload build
			let push_output = image::push_tag(
				ctx,
				&image::ImagePushTagOpts {
					tag: docker_image.clone(),
					name: Some(gen::display_name_from_date()),
					format: format.cloned(),
				},
			)
			.await?;
			docker.image_id = Some(push_output.image_id);
		}
	}

	Ok(())
}

pub async fn build_site(
	ctx: &cli_core::Ctx,
	cdn: &mut Box<models::CloudVersionCdnConfig>,
	format: Option<&struct_fmt::Format>,
) -> Result<()> {
	if cdn.site_id.is_none() {
		if let Some(build_output) = cdn.build_output.as_ref() {
			if let Some(build_command) = cdn.build_command.as_ref() {
				eprintln!();
				term::status::info("Building Site", build_command);

				if cfg!(unix) {
					let mut build_cmd = Command::new("/bin/sh");
					build_cmd.arg("-c").arg(build_command);
					let build_status = build_cmd.status().await?;
					ensure!(build_status.success(), "site failed to build");
				} else if cfg!(windows) {
					let mut build_cmd = Command::new("cmd.exe");
					build_cmd.arg("/C").arg(build_command);
					let build_status = build_cmd.status().await?;
					ensure!(build_status.success(), "site failed to build");
				} else {
					bail!("unknown machine type, expected unix or windows")
				};
			}

			// Upload site
			let push_output = site::push(
				ctx,
				&site::SitePushOpts {
					path: build_output.clone(),
					name: Some(gen::display_name_from_date()),
					format: format.cloned(),
				},
			)
			.await?;
			cdn.site_id = Some(push_output.site_id);
		}
	}

	Ok(())
}

pub fn dashboard_url(game_id: &str, version_id: &str) -> String {
	format!(
		"https://hub.rivet.gg/developer/games/{game_id}/versions/{version_id}",
		game_id = game_id,
		version_id = version_id
	)
}

pub fn rivet_game_url(game_name_id: &str, namespace_name_id: &str) -> String {
	if namespace_name_id == "prod" {
		format!("https://{game_name_id}.rivet.game/")
	} else {
		format!("https://{game_name_id}--{namespace_name_id}.rivet.game/")
	}
}

#[derive(Serialize)]
pub struct CreateOutput {
	pub version_id: Uuid,
}

/// Creates a new Rivet version.
pub async fn create(
	ctx: &cli_core::Ctx,
	display_name: Option<&str>,
	overrides: Vec<(String, serde_json::Value)>,
	namespace_name_id: Option<&str>,
	format: Option<&struct_fmt::Format>,
) -> Result<CreateOutput> {
	let display_name = display_name.map_or_else(gen::display_name_from_date, |x| x.to_string());
	// Fetch game data
	let game_res =
		cli_core::rivet_api::apis::cloud_games_games_api::cloud_games_games_get_game_by_id(
			&ctx.openapi_config_cloud,
			&ctx.game_id,
			None,
		)
		.await;
	if let Err(err) = game_res.as_ref() {
		println!("Error: {err:?}");
	}
	let game_res = game_res.context("cloud_games_games_get_game_by_id")?;
	let namespace = if let Some(namespace) = namespace_name_id {
		Some(
			game_res
				.game
				.namespaces
				.iter()
				.find(|x| x.name_id == namespace)
				.context("namespace not found")?,
		)
	} else {
		None
	};

	// Parse config
	let mut rivet_config = read_config(overrides, namespace_name_id).await?;
	build_config_dependencies(ctx, &mut rivet_config, format).await?;

	// Create game version
	let version_res =
		cli_core::rivet_api::apis::cloud_games_versions_api::cloud_games_versions_create_game_version(
			&ctx.openapi_config_cloud,
			&ctx.game_id,
			cli_core::rivet_api::models::CloudGamesCreateGameVersionRequest {
				display_name: display_name.clone(),
				config: Box::new(rivet_config),
			},
		)
		.await;
	if let Err(err) = version_res.as_ref() {
		println!("Error: {err:?}");
	}
	let version_res = version_res.context("versions_create_game_version")?;
	let version_id = version_res.version_id;

	eprintln!();
	term::status::success("Deployed Version", &display_name);
	term::status::info(
		"Version Dashboard",
		dashboard_url(&ctx.game_id, &version_id.to_string()),
	);

	// Deploy to namespace
	if let Some(namespace) = namespace {
		eprintln!();
		term::status::info(
			"Deploying to Namespace",
			format!("{} -> {}", display_name, namespace.display_name),
		);
		let update_version_res =
		cli_core::rivet_api::apis::cloud_games_namespaces_api::cloud_games_namespaces_update_game_namespace_version(
			&ctx.openapi_config_cloud,
			&ctx.game_id,
			&namespace.namespace_id.to_string(),
			cli_core::rivet_api::models::CloudGamesNamespacesUpdateGameNamespaceVersionRequest {
				version_id: version_id.clone()
			},
		)
		.await;
		if let Err(err) = update_version_res.as_ref() {
			println!("Error: {err:?}");
		}
		update_version_res.context("cloud_games_namespaces_update_game_namespace_version")?;
		term::status::success(
			"Deploy Succeeded",
			rivet_game_url(&game_res.game.name_id, &namespace.name_id),
		);
	}

	eprintln!();

	Ok(CreateOutput { version_id })
}

/// Backwards compatibility for site & Docker build pushing
///
/// Developers should use the parameters inside the config itself instead
pub async fn build_and_push_compat(
	ctx: &cli_core::Ctx,
	overrides: &mut Vec<(String, serde_json::Value)>,
	build_tag: &Option<String>,
	build_name: &Option<String>,
	site_path: &Option<String>,
	site_name: &Option<String>,
	format: &Option<struct_fmt::Format>,
) -> Result<()> {
	let site_output = if let Some(site_path) = site_path {
		Some(
			site::push(
				ctx,
				&site::SitePushOpts {
					path: site_path.clone(),
					name: site_name.clone(),
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
			image::push_tag(
				ctx,
				&image::ImagePushTagOpts {
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
