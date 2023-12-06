use anyhow::{bail, ensure, Context, Result};
use clap::Parser;
use cli_core::{ctx, rivet_api, Ctx};
use console::{style, Term};
use std::{
	path::{Path, PathBuf},
	str::FromStr,
};
use tokio::{fs, io::AsyncWriteExt};

use crate::{
	commands,
	util::{git, internal_config, term},
};

const CONFIG_DEFAULT_HEAD: &'static str = include_str!("../../tpl/default_config/head.yaml");
const CONFIG_DEFAULT_CDN: &'static str = include_str!("../../tpl/default_config/cdn.yaml");
const CONFIG_DEFAULT_MM: &'static str = include_str!("../../tpl/default_config/matchmaker.yaml");

const CONFIG_UNREAL: &'static str = include_str!("../../tpl/unreal_config/config.yaml");
const CONFIG_UNREAL_PROD: &'static str = include_str!("../../tpl/unreal_config/config-prod.yaml");

const UNREAL_DOCKERIGNORE: &'static str = include_str!("../../tpl/unreal_config/.dockerignore");
const UNREAL_SERVER_DEBUG_DOCKERFILE: &'static str =
	include_str!("../../tpl/unreal_config/server.debug.Dockerfile");
const UNREAL_SERVER_DEVELOPMENT_DOCKERFILE: &'static str =
	include_str!("../../tpl/unreal_config/server.development.Dockerfile");
const UNREAL_SERVER_SHIPPING_DOCKERFILE: &'static str =
	include_str!("../../tpl/unreal_config/server.shipping.Dockerfile");

#[derive(Clone, Copy)]
enum InitEngine {
	Unity,
	Unreal,
	Godot,
	HTML5,
	Custom,
}

impl InitEngine {
	fn learn_url(&self) -> String {
		match self {
			InitEngine::Unity => "https://rivet.gg/learn/unity".to_string(),
			InitEngine::Unreal => "https://rivet.gg/learn/unreal".to_string(),
			InitEngine::Godot => "https://rivet.gg/learn/godot".to_string(),
			InitEngine::HTML5 => "https://rivet.gg/learn/html5".to_string(),
			InitEngine::Custom => "https://rivet.gg/learn/custom".to_string(),
		}
	}
}

impl FromStr for InitEngine {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"unity" => Ok(InitEngine::Unity),
			"unreal" => Ok(InitEngine::Unreal),
			"godot" => Ok(InitEngine::Godot),
			"html5" => Ok(InitEngine::HTML5),
			"custom" => Ok(InitEngine::Custom),
			_ => bail!("Invalid engine"),
		}
	}
}

#[derive(Parser)]
pub struct Opts {
	#[clap(long)]
	recommend: bool,
	#[clap(long)]
	update_gitignore: bool,
	#[clap(long)]
	create_version_config: bool,
	#[clap(long)]
	install_plugin: bool,

	// Presets
	#[clap(long)]
	unity: bool,
	#[clap(long)]
	unreal: bool,
	#[clap(long)]
	godot: bool,
	#[clap(long)]
	html5: bool,
	#[clap(long)]
	custom: bool,

	// Matchmaker
	#[clap(long)]
	matchmaker: bool,
	#[clap(long)]
	matchmaker_port: Option<u16>,
	#[clap(long)]
	matchmaker_dockerfile: Option<String>,

	// CDN
	#[clap(long)]
	cdn: bool,
	#[clap(long)]
	cdn_build_command: Option<String>,
	#[clap(long)]
	cdn_build_output: Option<String>,

	// Dev
	#[clap(long)]
	dev: bool,
	#[clap(long)]
	dev_env: bool,
}

impl Opts {
	pub async fn execute(&self, term: &Term) -> Result<()> {
		let (api_endpoint, token) =
			internal_config::read(|x| (x.cluster.api_endpoint.clone(), x.tokens.cloud.clone()))
				.await?;
		let ctx = self
			.build_ctx(term, token.as_ref().map(|x| x.as_str()), api_endpoint)
			.await?;

		// Select the engine to use
		let init_engine = if self.unity {
			InitEngine::Unity
		} else if self.unreal {
			InitEngine::Unreal
		} else if self.godot {
			InitEngine::Godot
		} else if self.html5 {
			InitEngine::HTML5
		} else if self.custom {
			InitEngine::Custom
		} else {
			let engine = term::Prompt::new("What engine are you using?")
				.docs("unity, unreal, godot, html5, or custom")
				.default_value("custom")
				.parsed::<InitEngine>(term)
				.await?;
			engine
		};

		// Run setup process
		match init_engine {
			InitEngine::Unreal => {
				self.create_config_unreal(term).await?;
			}
			_ => {
				// TODO: Add setup process for Unity & Godot & HTML5
				// Default pipeline
				let has_version_config = self.create_config_default(term, init_engine).await?;
				self.create_dev_token(term, &ctx, has_version_config)
					.await?;
			}
		}

		self.update_gitignore(term).await?;

		eprintln!();
		term::status::success("What's next?", init_engine.learn_url());

		Ok(())
	}

	async fn build_ctx(
		&self,
		term: &Term,
		token: Option<&str>,
		override_endpoint: Option<String>,
	) -> Result<Ctx> {
		// Check if token already exists
		let token = if let Some(token) = token.clone() {
			Some(token.to_string())
		} else {
			internal_config::read(|x| x.tokens.cloud.clone()).await?
		};
		let ctx = if let Some(token) = token {
			let ctx = cli_core::ctx::init(override_endpoint.clone(), token).await?;

			let game_res =
				rivet_api::apis::cloud_games_games_api::cloud_games_games_get_game_by_id(
					&ctx.openapi_config_cloud,
					&ctx.game_id,
					None,
				)
				.await
				.context("cloud_games_games_get_game_by_id")?;
			let display_name = game_res.game.display_name;

			term::status::success("Found existing token", display_name);

			ctx
		} else {
			read_token(term, override_endpoint.clone()).await?
		};

		Ok(ctx)
	}

	async fn update_gitignore(&self, term: &Term) -> Result<()> {
		if !git::check_ignore(Path::new(".rivet/")).await? {
			if self.recommend
				|| self.update_gitignore
				|| term::Prompt::new("Add .rivet/ to .gitignore?")
					.docs(".rivet/ holds secrets and local configuration files that should not be version controlled")
					.docs_url("https://rivet.gg/docs/general/concepts/dot-rivet-directory")
					.default_value("yes")
					.bool(term).await?
			{
				let mut file = fs::OpenOptions::new()
					.write(true)
					.append(true)
					.open(".gitignore")
					.await?;
				file.write_all(b"\n### Rivet ###\n.rivet/\n.env\n").await?;

				ensure!(
					git::check_ignore(Path::new(".rivet/")).await?,
					"updated gitignore does not ignore Rivet files"
				);

				term::status::success("Finished", "Git will now ignore the .rivet/ folder");
			}
		} else {
			term::status::success(
				".gitignore already configured",
				"The .rivet/ folder is already ignored by Git",
			);
		}

		Ok(())
	}

	async fn create_config_unreal(&self, term: &Term) -> Result<()> {
		let dockerignore_path = std::env::current_dir()?.join(".dockerignore");
		let dockerfile_dev_path = std::env::current_dir()?.join("server.development.Dockerfile");
		let dockerfile_debug_path = std::env::current_dir()?.join("server.debug.Dockerfile");
		let dockerfile_shipping_path = std::env::current_dir()?.join("server.shipping.Dockerfile");
		let config_path = std::env::current_dir()?.join("rivet.yaml");
		let config_prod_path = std::env::current_dir()?.join("rivet.prod.yaml");

		// Build the uproject path
		let current_dir = std::env::current_dir()?;
		let uproject_path = find_uproject_file(&current_dir)
			.await
			.context("find_uproject_file")?
			.context("could not find *.uproject file")?;
		let uproject_path_unix = uproject_path
			.strip_prefix(current_dir)
			.context("failed to strip uproject path prefix")?
			.components()
			.map(|c| c.as_os_str().to_string_lossy())
			.collect::<Vec<_>>()
			.join("/");

		// Read module name
		let mut module_name_prompt = term::Prompt::new("Unreal game module name?").docs("Name of the Unreal module that holds the game code. This is usually the value of `$.Modules[0].Name` in the file `MyProject.unproject`.");
		if let Some(module_name) = attempt_read_module_name(&uproject_path).await? {
			module_name_prompt = module_name_prompt.default_value(module_name);
		}
		let game_module = module_name_prompt.string(term).await?;

		// Generate Dockerfiles
		let mut dockerfile_created = false;
		if !fs::try_exists(&dockerignore_path).await? {
			fs::write(&dockerignore_path, UNREAL_DOCKERIGNORE).await?;
			term::status::success("Created .dockerignore", "");
		}
		if !fs::try_exists(&dockerfile_dev_path).await? {
			fs::write(
				&dockerfile_dev_path,
				UNREAL_SERVER_DEVELOPMENT_DOCKERFILE
					.replace("__UPROJECT_PATH__", &uproject_path_unix)
					.replace("__GAME_MODULE__", &game_module),
			)
			.await?;
			term::status::success("Created server.development.Dockerfile", "");
			dockerfile_created = true;
		}
		if !fs::try_exists(&dockerfile_debug_path).await? {
			fs::write(
				&dockerfile_debug_path,
				UNREAL_SERVER_DEBUG_DOCKERFILE
					.replace("__UPROJECT_PATH__", &uproject_path_unix)
					.replace("__GAME_MODULE__", &game_module),
			)
			.await?;
			term::status::success("Created server.debug.Dockerfile", "");
			dockerfile_created = true;
		}
		if !fs::try_exists(&dockerfile_shipping_path).await? {
			fs::write(
				&dockerfile_shipping_path,
				UNREAL_SERVER_SHIPPING_DOCKERFILE
					.replace("__UPROJECT_PATH__", &uproject_path_unix)
					.replace("__GAME_MODULE__", &game_module),
			)
			.await?;
			term::status::success("Created server.shipping.Dockerfile", "");
			dockerfile_created = true;
		}
		if !dockerfile_created {
			term::status::success(
				"Dockerfiles already created",
				"Your game already has server.*.Dockerfile",
			);
		}

		// Generate config file
		let mut config_created = false;
		if self.create_version_config || !fs::try_exists(&config_path).await? {
			let version_config = CONFIG_UNREAL.replace("__GAME_MODULE__", &game_module);
			fs::write(&config_path, version_config).await?;
			term::status::success("Created rivet.yaml", "");
			config_created = true;
		}
		if self.create_version_config || !fs::try_exists(&config_prod_path).await? {
			fs::write(&config_prod_path, CONFIG_UNREAL_PROD).await?;
			term::status::success("Created rivet.prod.yaml", "");
			config_created = true;
		}
		if !config_created {
			term::status::success(
				"Version already configured",
				"Your game is already configured with rivet.yaml",
			);
		}

		// Install plugin
		if self.recommend
			|| self.install_plugin
			|| term::Prompt::new("Install or upgrade Unreal Engine Rivet plugin?")
				.docs("This plugin is used to integrate your game with Rivet")
				.docs_url("https://github.com/rivet-gg/plugin-unreal")
				.default_value("yes")
				.bool(term)
				.await?
		{
			commands::engine::unreal::install_plugin().await?;
		}

		Ok(())
	}

	async fn create_config_default(&self, term: &Term, init_engine: InitEngine) -> Result<bool> {
		let config_path = std::env::current_dir()?.join("rivet.yaml");
		let config_needs_creation = match fs::read_to_string(&config_path).await {
			Ok(_) => false,
			Err(err) if err.kind() == std::io::ErrorKind::NotFound => true,
			Err(err) => {
				return Err(err.into());
			}
		};
		let has_version_config = if config_needs_creation {
			if self.recommend
				|| self.create_version_config
				|| term::Prompt::new("Create rivet.yaml?")
					.docs("This is the configuration file used to manage your game")
					.docs_url("https://rivet.gg/docs/general/concepts/version-config")
					.default_value("yes")
					.bool(term)
					.await?
			{
				let mut version_config = CONFIG_DEFAULT_HEAD.to_string();

				// Add engine config
				match init_engine {
					InitEngine::Unity => {
						version_config.push_str("[engine.unity]\n");
					}
					InitEngine::Unreal => {
						version_config.push_str("[engine.unreal]\n");
					}
					InitEngine::Godot => {
						version_config.push_str("[engine.godot]\n");
					}
					InitEngine::HTML5 => {
						version_config.push_str("[engine.html5]\n");
					}
					InitEngine::Custom => {
						// Do nothing
					}
				}

				if self.matchmaker
					|| term::Prompt::new("Enable Rivet Matchmaker?")
						.indent(1)
						.context("rivet.yaml")
						.docs("Setup your matchmaker configuration, this can be changed later")
						.docs_url("https://rivet.gg/docs/matchmaker")
						.default_value("yes")
						.bool(term)
						.await?
				{
					let port = if let Some(port) = &self.matchmaker_port {
						*port
					} else {
						term::Prompt::new("What port does your game server listen on?")
							.indent(2)
							.context("Matchmaker")
							.default_value("8080")
							.parsed::<u16>(term)
							.await?
					};

					let mut dockerfile_path = if let Some(dockerfile) = &self.matchmaker_dockerfile
					{
						dockerfile.clone()
					} else {
						term::Prompt::new("Path to the server's Dockerfile?")
							.indent(2)
							.context("Matchmaker")
							.default_value("Dockerfile")
							.string(term)
							.await?
					};
					if dockerfile_path.is_empty() {
						dockerfile_path = "Dockerfile".to_string();
					}

					version_config.push_str(
						&CONFIG_DEFAULT_MM
							.replace("__DOCKERFILE__", &dockerfile_path)
							.replace("__PORT__", &port.to_string()),
					);
				}

				if self.cdn
					|| term::Prompt::new("Enable Rivet CDN?")
						.indent(1)
						.context("rivet.yaml")
						.docs("Setup service a website or static assets, this can be changed later")
						.docs_url("https://rivet.gg/docs/cdn")
						.default_value("yes")
						.bool(term)
						.await?
				{
					let mut build_command = if let Some(build_command) = &self.cdn_build_command {
						build_command.clone()
					} else {
						term::Prompt::new("What command will run before uploading your site?")
							.indent(2)
							.context("CDN")
							.default_value("echo 'Nothing to do'")
							.string(term)
							.await?
					};
					if build_command.is_empty() {
						build_command = "echo 'Nothing to do'".to_string();
					}

					let mut build_output = if let Some(build_output) = &self.cdn_build_output {
						build_output.clone()
					} else {
						term::Prompt::new("What directory should be uploaded to Rivet CDN?")
							.indent(2)
							.context("CDN")
							.default_value("dist/")
							.string(term)
							.await?
					};
					if build_output.is_empty() {
						build_output = "dist/".to_string();
					}

					version_config.push_str(
						&CONFIG_DEFAULT_CDN
							.replace("__BUILD_COMMAND__", &build_command.replace("\"", "\\\""))
							.replace("__BUILD_OUTPUT__", &build_output),
					);
				}

				// Write file
				fs::write(config_path, version_config).await?;

				term::status::success("Created rivet.yaml", "");

				true
			} else {
				false
			}
		} else {
			term::status::success(
				"Version already configured",
				"Your game is already configured with rivet.yaml",
			);
			true
		};

		Ok(has_version_config)
	}

	async fn create_dev_token(
		&self,
		term: &Term,
		ctx: &Ctx,
		has_version_config: bool,
	) -> Result<()> {
		if has_version_config
			&& commands::version::read_config(Vec::new(), None)
				.await?
				.matchmaker
				.is_some() && (self.recommend
			|| self.dev || term::Prompt::new("Setup development environment?")
			.docs("Create development tokens that enable you to develop your game locally")
			.docs_url("http://rivet.gg/docs/general/concepts/dev-tokens")
			.bool(term)
			.await?)
		{
			commands::token::create::dev::Opts {
				dev_env: true,
				namespace: None,
			}
			.execute(&ctx)
			.await?
		}

		Ok(())
	}
}

async fn read_token(term: &Term, override_endpoint: Option<String>) -> Result<cli_core::Ctx> {
	// Create OpenAPI configuration without bearer token to send link request
	let openapi_config_cloud_unauthed = rivet_api::apis::configuration::Configuration {
		base_path: override_endpoint
			.clone()
			.unwrap_or_else(|| ctx::DEFAULT_API_ENDPOINT.to_string()),
		user_agent: Some(ctx::user_agent()),
		..Default::default()
	};

	// Prepare the link
	let prepare_res = rivet_api::apis::cloud_devices_links_api::cloud_devices_links_prepare(
		&openapi_config_cloud_unauthed,
	)
	.await;
	if let Err(err) = prepare_res.as_ref() {
		println!("Error: {err:?}");
	}
	let prepare_res = prepare_res.context("cloud_devices_links_prepare")?;

	// Prompt user to press enter to open browser
	term::status::info("Link your game", "Press Enter to open your browser");
	tokio::task::spawn_blocking({
		let term = term.clone();
		move || term.read_char()
	})
	.await??;

	// Open link in browser
	if webbrowser::open_browser_with_options(
		webbrowser::Browser::Default,
		&prepare_res.device_link_url,
		webbrowser::BrowserOptions::new().with_suppress_output(true),
	)
	.is_ok()
	{
		term::status::info(
			"Waiting for link",
			"Select the game to link in your browser",
		);
	} else {
		eprintln!(
			"{}\n  {}",
			style("Visit the link below").bold().blue(),
			style(&prepare_res.device_link_url)
				.italic()
				.underlined()
				.cyan()
		);
	}

	// Wait for link to complete
	let mut watch_index = None;
	let token = loop {
		let prepare_res = rivet_api::apis::cloud_devices_links_api::cloud_devices_links_get(
			&openapi_config_cloud_unauthed,
			&prepare_res.device_link_token,
			watch_index.as_ref().map(String::as_str),
		)
		.await;
		if let Err(err) = prepare_res.as_ref() {
			println!("Error: {err:?}");
		}
		let prepare_res = prepare_res.context("cloud_devices_links_get")?;

		watch_index = Some(prepare_res.watch.index);

		if let Some(token) = prepare_res.cloud_token {
			break token;
		}
	};

	// Create new context
	let new_ctx = cli_core::ctx::init(
		override_endpoint,
		// Exclude overridden access token to check the token
		token.clone(),
	)
	.await?;

	// Inspect the token
	let inspect_res =
		rivet_api::apis::cloud_auth_api::cloud_auth_inspect(&new_ctx.openapi_config_cloud).await;
	if let Err(err) = inspect_res.as_ref() {
		println!("Error: {err:?}");
	}
	let inspect_res = inspect_res.context("cloud_auth_inspect")?;

	// Find the game ID
	let Some(game_cloud) = inspect_res.agent.game_cloud.as_ref() else {
		bail!("token is not a GameCloud token")
	};
	let game_id = game_cloud.game_id;

	// Extract game data
	let game_res = rivet_api::apis::cloud_games_games_api::cloud_games_games_get_game_by_id(
		&new_ctx.openapi_config_cloud,
		&game_id.to_string(),
		None,
	)
	.await;
	if let Err(err) = game_res.as_ref() {
		println!("Error: {err:?}");
	}
	let game_res = game_res.context("cloud_games_games_get_game_by_id")?;
	let display_name = game_res.game.display_name;

	// Write the token
	internal_config::mutate(|x| x.tokens.cloud = Some(token)).await?;

	term::status::success("Token Saved", display_name);

	Ok(new_ctx)
}

/// Finds the Unreal project file in the current directory.
async fn find_uproject_file(current_dir: &Path) -> Result<Option<PathBuf>> {
	let mut read_dir = fs::read_dir(current_dir).await?;
	while let Some(entry) = read_dir.next_entry().await? {
		let path = entry.path();
		if let Some(ext) = path.extension() {
			if ext == "uproject" {
				return Ok(Some(path));
			}
		}
	}

	Ok(None)
}

/// Attempts to read the module name from the uproject file.
async fn attempt_read_module_name(uproject_path: &Path) -> Result<Option<String>> {
	// Read uproject file
	let uproject_str = match fs::read_to_string(&uproject_path).await {
		Ok(uproject) => uproject,
		Err(_) => {
			return Ok(None);
		}
	};
	let uproject_json = match serde_json::from_str::<serde_json::Value>(&uproject_str) {
		Ok(uproject_json) => uproject_json,
		Err(_) => {
			return Ok(None);
		}
	};

	// Extract module name
	let project_name = uproject_json
		.get("Modules")
		.and_then(|x| x.get(0))
		.and_then(|x| x.get("Name"))
		.and_then(|x| x.as_str())
		.map(|x| x.to_string());

	Ok(project_name)
}
