use anyhow::{bail, Context, Result};
use clap::Parser;
use cli_core::{ctx, rivet_api::apis, Ctx};
use console::{style, Color, Style, Term};
use std::{
	path::{Path, PathBuf},
	str::FromStr,
};
use tokio::fs;

use crate::{
	commands,
	util::{global_config, paths, term},
};

const CONFIG_DEFAULT_HEAD: &'static str = include_str!("../../tpl/default_config/head.yaml");

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
	unity: bool,
	#[clap(long)]
	unreal: bool,
	#[clap(long)]
	godot: bool,
	#[clap(long)]
	html5: bool,
	#[clap(long)]
	custom: bool,
}

impl Opts {
	pub async fn execute(&self, term: &Term) -> Result<()> {
		// Remove legacy `.rivet` dir if exists
		let legacy_project_meta_path = paths::project_root()?.join(".rivet");
		if fs::metadata(&legacy_project_meta_path).await.is_ok() {
			term::status::warn(
				"Deleting legacy project metadata",
				".rivet/ folder is moved to a global config",
			);
			fs::remove_dir_all(&legacy_project_meta_path).await?;
		}

		// Build context
		let (api_endpoint, token) = global_config::read_project(|x| {
			(x.cluster.api_endpoint.clone(), x.tokens.cloud.clone())
		})
		.await?;
		let _ctx = self
			.init_ctx_and_token(term, token.as_ref().map(|x| x.as_str()), api_endpoint)
			.await?;

		// Attempt to read the existing config
		let partial_config = commands::config::read_config_partial(Vec::new(), None)
			.await
			.ok();

		// Read the engine from the existing config
		let init_engine =
			if let Some(engine) = partial_config.as_ref().and_then(|x| x.engine.as_ref()) {
				if engine.unity.is_some() {
					Some(InitEngine::Unity)
				} else if engine.unreal.is_some() {
					Some(InitEngine::Unreal)
				} else if engine.godot.is_some() {
					Some(InitEngine::Godot)
				} else if engine.html5.is_some() {
					Some(InitEngine::HTML5)
				} else {
					None
				}
			} else {
				None
			};

		// Prompt for engine if not provided
		let init_engine = if let Some(x) = init_engine {
			x
		} else {
			if self.unity {
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
			}
		};

		// Run setup process
		match init_engine {
			InitEngine::Unreal => {
				self.create_config_unreal(term).await?;
			}
			_ => {
				// TODO: Add setup process for Unity & Godot & HTML5
				// Default pipeline
				self.create_config_default(init_engine).await?;
			}
		}

		let width = term.size_checked().map(|x| x.1).unwrap_or(80) as usize;

		eprintln!();
		eprintln!();
		eprintln!("{}", style(center_text("Welcome to", width)).bold());
		eprintln!(
			"{}",
			center_text(include_str!("../../tpl/graphics/logo.txt"), width)
		);
		eprintln!();
		eprintln!(
			"{}",
			style(center_text("Riveting Experiences", width))
				.italic()
				.dim()
		);
		eprintln!();
		rainbow_line(width);
		eprintln!();
		eprintln!(
			"{}",
			center_text(include_str!("../../tpl/graphics/get-started.txt"), width)
		);
		eprintln!();

		Ok(())
	}

	/// Gets or creates the cloud token & creates an initial context.
	async fn init_ctx_and_token(
		&self,
		term: &Term,
		token: Option<&str>,
		override_endpoint: Option<String>,
	) -> Result<Ctx> {
		// Check if token already exists
		let token = if let Some(token) = token.clone() {
			Some(token.to_string())
		} else {
			global_config::read_project(|x| x.tokens.cloud.clone()).await?
		};
		let ctx = if let Some(token) = token {
			let ctx = cli_core::ctx::init(override_endpoint.clone(), token).await?;

			let game_res = apis::cloud_games_games_api::cloud_games_games_get_game_by_id(
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
		if fs::try_exists(&config_path).await? {
			let mut version_config =
				CONFIG_DEFAULT_HEAD.replace("__LEARN_URL__", "https://rivet.gg/learn/unreal");
			version_config.push_str(&CONFIG_UNREAL.replace("__GAME_MODULE__", &game_module));
			fs::write(&config_path, version_config).await?;

			eprintln!();
			term::status::success(
				"Created rivet.yaml",
				"https://rivet.gg/docs/general/concepts/version-config",
			);
			config_created = true;

			// Only create prod config if no default config already exists
			if fs::try_exists(&config_prod_path).await? {
				fs::write(&config_prod_path, CONFIG_UNREAL_PROD).await?;
				term::status::success("Created rivet.prod.yaml", "");
				config_created = true;
			}
		}
		if !config_created {
			term::status::success(
				"Version already configured",
				"Your game is already configured with rivet.yaml",
			);
		}

		// Install plugin
		if term::Prompt::new("Install or upgrade Unreal Engine Rivet plugin?")
				.docs("This plugin is used to integrate your game with Rivet. This can be done later with `rivet unreal install-plugin`")
				.docs_url("https://github.com/rivet-gg/plugin-unreal")
				.default_value("yes")
				.bool(term)
				.await?
		{
			commands::engine::unreal::install_plugin().await?;
		}

		Ok(())
	}

	async fn create_config_default(&self, init_engine: InitEngine) -> Result<bool> {
		let current_dir = std::env::current_dir()?;
		let config_exists = ["rivet.yaml", "rivet.toml", "rivet.json"]
			.iter()
			.any(|file_name| current_dir.join(file_name).exists());
		let has_version_config = if !config_exists {
			let mut version_config =
				CONFIG_DEFAULT_HEAD.replace("__LEARN_URL__", &init_engine.learn_url());

			// Add engine config
			match init_engine {
				InitEngine::Unity => {
					version_config.push_str("engine:\n  unity: {}\n\n");
				}
				InitEngine::Unreal => {
					version_config.push_str("engine:\n  unreal: {}\n\n");
				}
				InitEngine::Godot => {
					version_config.push_str("engine:\n  godot: {}\n\n");
				}
				InitEngine::HTML5 => {
					version_config.push_str("engine:\n  html5: {}\n\n");
				}
				InitEngine::Custom => {
					// Do nothing
				}
			}

			// Write file
			fs::write(current_dir.join("rivet.yaml"), version_config).await?;

			eprintln!();
			term::status::success(
				"Created rivet.yaml",
				"https://rivet.gg/docs/general/concepts/version-config",
			);

			true
		} else {
			term::status::success(
				"Version already configured",
				"Your game is already configured with rivet.yaml",
			);
			true
		};

		Ok(has_version_config)
	}
}

async fn read_token(term: &Term, override_endpoint: Option<String>) -> Result<cli_core::Ctx> {
	// Create OpenAPI configuration without bearer token to send link request
	let openapi_config_cloud_unauthed = apis::configuration::Configuration {
		base_path: override_endpoint
			.clone()
			.unwrap_or_else(|| ctx::DEFAULT_API_ENDPOINT.to_string()),
		user_agent: Some(ctx::user_agent()),
		..Default::default()
	};

	// Prepare the link
	let prepare_res =
		apis::cloud_devices_links_api::cloud_devices_links_prepare(&openapi_config_cloud_unauthed)
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
		let prepare_res = apis::cloud_devices_links_api::cloud_devices_links_get(
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
	let inspect_res = apis::cloud_auth_api::cloud_auth_inspect(&new_ctx.openapi_config_cloud).await;
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
	let game_res = apis::cloud_games_games_api::cloud_games_games_get_game_by_id(
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
	global_config::mutate_project(|x| x.tokens.cloud = Some(token)).await?;

	term::status::success("Linked Game", display_name);

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

fn center_text(input: &str, width: usize) -> String {
	let longest_len = input
		.lines()
		.map(|line| line.chars().count())
		.max()
		.unwrap_or(0);

	let padding = (width.saturating_sub(longest_len)) / 2;
	let padding_str = " ".repeat(padding);

	input
		.lines()
		.map(|line| format!("{padding_str}{line}"))
		.collect::<Vec<String>>()
		.join("\n")
}

fn rainbow_line(len: usize) {
	let term = Term::stdout();
	let colors = [
		Color::Red,
		Color::Yellow,
		Color::Green,
		Color::Cyan,
		Color::Blue,
		Color::Magenta,
	];
	let block_len = len / colors.len();
	let block = "█".repeat(block_len);

	// Print rainbow lines
	for &color in colors.iter() {
		let styled = Style::new().fg(color).apply_to(&block);
		term.write_str(&styled.to_string()).unwrap();
	}

	// Fill the remaining blocks
	let remaining = len % (colors.len() * block_len);
	for _ in 0..remaining {
		let styled = Style::new()
			.fg(colors.last().unwrap().clone())
			.apply_to("█");
		term.write_str(&styled.to_string()).unwrap();
	}

	term.write_str("\n").unwrap();
}
