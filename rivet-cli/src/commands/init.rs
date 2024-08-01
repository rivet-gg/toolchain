use clap::Parser;
use toolchain_core::{
	ctx,
	rivet_api::{apis, models},
	Ctx,
};
use console::{style, Term};
use global_error::prelude::*;

use std::path::{Path, PathBuf};
use tokio::fs;

use crate::{
	commands,
	util::{global_config, os, paths, text, version_config::Engine},
};

const CONFIG_UNREAL: &'static str = include_str!("../../tpl/unreal_config/config.yaml");
const CONFIG_UNREAL_PROD: &'static str = include_str!("../../tpl/unreal_config/config-prod.yaml");

const UNREAL_DOCKERIGNORE: &'static str = include_str!("../../tpl/unreal_config/.dockerignore");
const UNREAL_SERVER_DEBUG_DOCKERFILE: &'static str =
	include_str!("../../tpl/unreal_config/server.debug.Dockerfile");
const UNREAL_SERVER_DEVELOPMENT_DOCKERFILE: &'static str =
	include_str!("../../tpl/unreal_config/server.development.Dockerfile");
const UNREAL_SERVER_SHIPPING_DOCKERFILE: &'static str =
	include_str!("../../tpl/unreal_config/server.shipping.Dockerfile");

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
	pub async fn execute(&self, term: &Term) -> GlobalResult<()> {
		// Remove legacy `.rivet` dir if exists
		let legacy_project_meta_path = paths::project_root()?.join(".rivet");
		if fs::metadata(&legacy_project_meta_path).await.is_ok() {
			rivet_term::status::warn(
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
		let ctx =
			init_ctx_and_token(term, token.as_ref().map(|x| x.as_str()), api_endpoint).await?;

		// Read game
		let game_res = unwrap!(
			apis::cloud_games_api::cloud_games_get_game_by_id(
				&ctx.openapi_config_cloud,
				&ctx.game_id,
				None,
			)
			.await
		);

		// Attempt to read the existing config
		let partial_config = commands::config::read_config_partial(Vec::new(), None)
			.await
			.ok();

		// Read the engine from the existing config
		let engine = if let Some(engine) = partial_config.as_ref().and_then(|x| x.engine.as_ref()) {
			if engine.unity.is_some() {
				Some(Engine::Unity)
			} else if engine.unreal.is_some() {
				Some(Engine::Unreal)
			} else if engine.godot.is_some() {
				Some(Engine::Godot)
			} else if engine.html5.is_some() {
				Some(Engine::Html5)
			} else if engine.custom.is_some() {
				Some(Engine::Custom)
			} else {
				None
			}
		} else {
			None
		};

		// Prompt for engine if not provided
		let engine = if let Some(x) = engine {
			x
		} else {
			if self.unity {
				Engine::Unity
			} else if self.unreal {
				Engine::Unreal
			} else if self.godot {
				Engine::Godot
			} else if self.html5 {
				Engine::Html5
			} else if self.custom {
				Engine::Custom
			} else {
				let engine = rivet_term::prompt::PromptBuilder::default()
					.message("What engine are you using?")
					.docs("unity, unreal, godot, html5, or custom")
					.build()?
					.parsed::<Engine>(term)
					.await?;
				engine
			}
		};

		// Create config
		let created_config = match &engine {
			Engine::Unreal => create_config_unreal(term).await?,
			_ => {
				// Default pipeline
				create_config_default(&engine).await?
			}
		};

		// Print welcome
		print_welcome(
			&WelcomeContext {
				game: game_res.game,
				engine,
				created_config,
			},
			term,
		);

		Ok(())
	}
}

/// Gets or creates the cloud token & creates an initial context.
async fn init_ctx_and_token(
	term: &Term,
	token: Option<&str>,
	override_endpoint: Option<String>,
) -> GlobalResult<Ctx> {
	// Check if token already exists
	let token = if let Some(token) = token.clone() {
		Some(token.to_string())
	} else {
		global_config::read_project(|x| x.tokens.cloud.clone()).await?
	};
	let ctx = if let Some(token) = token {
		toolchain_core::ctx::init(override_endpoint.clone(), token).await?
	} else {
		read_token(term, override_endpoint.clone()).await?
	};

	Ok(ctx)
}

async fn create_config_unreal(term: &Term) -> GlobalResult<bool> {
	let dockerignore_path = std::env::current_dir()?.join(".dockerignore");
	let dockerfile_dev_path = std::env::current_dir()?.join("server.development.Dockerfile");
	let dockerfile_debug_path = std::env::current_dir()?.join("server.debug.Dockerfile");
	let dockerfile_shipping_path = std::env::current_dir()?.join("server.shipping.Dockerfile");
	let config_path = std::env::current_dir()?.join("rivet.yaml");
	let config_prod_path = std::env::current_dir()?.join("rivet.prod.yaml");

	// Build the uproject path
	let current_dir = std::env::current_dir()?;
	let uproject_path = unwrap!(
		unwrap!(
			find_uproject_file(&current_dir).await,
			"could not find *.uproject file"
		),
		"could not find *.uproject file"
	);
	let uproject_path_unix = unwrap!(
		uproject_path.strip_prefix(current_dir),
		"failed to strip uproject path prefix"
	)
	.components()
	.map(|c| c.as_os_str().to_string_lossy())
	.collect::<Vec<_>>()
	.join("/");

	// Read module name
	let mut module_name_prompt = rivet_term::prompt::PromptBuilder::default();
	module_name_prompt.message("Unreal game module name?");
	module_name_prompt.docs("Name of the Unreal module that holds the game code. This is usually the value of `$.Modules[0].Name` in the file `MyProject.unproject`.");

	if let Some(module_name) = attempt_read_module_name(&uproject_path).await? {
		module_name_prompt.default_value(module_name);
	}
	let game_module = module_name_prompt.build()?.string(term).await?;

	// Generate Dockerfiles
	let mut dockerfile_created = false;
	if !fs::try_exists(&dockerignore_path).await? {
		fs::write(&dockerignore_path, UNREAL_DOCKERIGNORE).await?;
		rivet_term::status::success("Created .dockerignore", "");
	}
	if !fs::try_exists(&dockerfile_dev_path).await? {
		fs::write(
			&dockerfile_dev_path,
			UNREAL_SERVER_DEVELOPMENT_DOCKERFILE
				.replace("__UPROJECT_PATH__", &uproject_path_unix)
				.replace("__GAME_MODULE__", &game_module),
		)
		.await?;
		rivet_term::status::success("Created server.development.Dockerfile", "");
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
		rivet_term::status::success("Created server.debug.Dockerfile", "");
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
		rivet_term::status::success("Created server.shipping.Dockerfile", "");
		dockerfile_created = true;
	}
	if !dockerfile_created {
		rivet_term::status::success(
			"Dockerfiles already created",
			"Your game already has server.*.Dockerfile",
		);
	}

	// Generate config file
	let mut created_config = false;
	if fs::try_exists(&config_path).await? {
		let mut version_config = crate::util::version_config::generate(&Engine::Unreal, false)?;
		version_config.push_str(&CONFIG_UNREAL.replace("__GAME_MODULE__", &game_module));
		fs::write(&config_path, version_config).await?;

		eprintln!();
		rivet_term::status::success("Created rivet.yaml", "https://rivet.gg/docs/general/config");
		created_config = true;

		// Only create prod config if no default config already exists
		if fs::try_exists(&config_prod_path).await? {
			fs::write(&config_prod_path, CONFIG_UNREAL_PROD).await?;
			rivet_term::status::success("Created rivet.prod.yaml", "");
			created_config = true;
		}
	}
	if !created_config {
		rivet_term::status::success(
			"Version already configured",
			"Your game is already configured with rivet.yaml",
		);
	}

	// Install plugin
	if rivet_term::prompt::PromptBuilder::default()
			.message("Install or upgrade Unreal Engine Rivet plugin?")
			.docs("This plugin is used to integrate your game with Rivet. This can be done later with `rivet unreal install-plugin`")
			.docs_url("https://github.com/rivet-gg/plugin-unreal")
			.default_value("yes")
			.build()?
			.bool(term)
			.await?
		{
			commands::engine::unreal::install_plugin().await?;
		}

	Ok(created_config)
}

pub async fn create_config_default(engine: &Engine) -> GlobalResult<bool> {
	let current_dir = std::env::current_dir()?;
	let config_exists = ["rivet.yaml", "rivet.toml", "rivet.json"]
		.iter()
		.any(|file_name| current_dir.join(file_name).exists());
	let created_config = if !config_exists {
		// Write config
		let version_config = crate::util::version_config::generate(&engine, true)?;
		fs::write(current_dir.join("rivet.yaml"), version_config).await?;

		// eprintln!();
		// rivet_term::status::success(
		// 	"Created rivet.yaml",
		// 	"https://rivet.gg/docs/general/concepts/config",
		// );

		true
	} else {
		// rivet_term::status::success(
		// 	"Version already configured",
		// 	"Your game is already configured with rivet.yaml",
		// );
		false
	};

	Ok(created_config)
}

async fn read_token(term: &Term, override_endpoint: Option<String>) -> GlobalResult<toolchain_core::Ctx> {
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
	let prepare_res = prepare_res?;

	// Prompt user to press enter to open browser
	rivet_term::status::info("Link your game", "Press Enter to open your browser");
	tokio::task::spawn_blocking({
		let term = term.clone();
		move || term.read_char()
	})
	.await??;

	// Open link in browser
	//
	// Linux root users often cannot open the browser, so we fallback to printing the URL
	if !os::is_linux_and_root()
		&& webbrowser::open_browser_with_options(
			webbrowser::Browser::Default,
			&prepare_res.device_link_url,
			webbrowser::BrowserOptions::new().with_suppress_output(true),
		)
		.is_ok()
	{
		rivet_term::status::info(
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
		let prepare_res = unwrap!(prepare_res);

		watch_index = Some(prepare_res.watch.index);

		if let Some(token) = prepare_res.cloud_token {
			break token;
		}
	};

	// Create new context
	let new_ctx = toolchain_core::ctx::init(
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
	let inspect_res = unwrap!(inspect_res);

	// Find the game ID
	let Some(game_cloud) = inspect_res.agent.game_cloud.as_ref() else {
		bail!("token is not a GameCloud token")
	};
	let game_id = game_cloud.game_id;

	// Extract game data
	let game_res = apis::cloud_games_api::cloud_games_get_game_by_id(
		&new_ctx.openapi_config_cloud,
		&game_id.to_string(),
		None,
	)
	.await;
	if let Err(err) = game_res.as_ref() {
		println!("Error: {err:?}");
	}
	let game_res = unwrap!(game_res);
	let display_name = game_res.game.display_name;

	// Write the token
	global_config::mutate_project(|x| x.tokens.cloud = Some(token)).await?;

	rivet_term::status::success("Linked Game", display_name);

	Ok(new_ctx)
}

/// Finds the Unreal project file in the current directory.
async fn find_uproject_file(current_dir: &Path) -> GlobalResult<Option<PathBuf>> {
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
async fn attempt_read_module_name(uproject_path: &Path) -> GlobalResult<Option<String>> {
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

/// Context required to display welcome message.
struct WelcomeContext {
	game: Box<models::CloudGameFull>,
	engine: Engine,
	created_config: bool,
}

fn print_welcome(welcome_context: &WelcomeContext, term: &Term) {
	let width = term.size_checked().map(|x| x.1).unwrap_or(80) as usize;

	eprintln!();
	eprintln!();
	eprintln!("{}", style(text::center_text("Welcome to", width)).bold());
	eprintln!(
		"{}",
		text::center_text(include_str!("../../tpl/graphics/logo.txt"), width)
	);
	eprintln!(
		"{}",
		style(text::center_text("Riveting Experiences", width))
			.italic()
			.dim()
	);
	eprintln!();
	eprintln!();
	text::rainbow_line(width);
	eprintln!();
	eprintln!(
		"{}",
		text::center_text(
			&text::render_box_padded(
				&format!(
					"\nLinked to {game}\n\n{middle}\n\n{created_config}\n\n{middle}\n\nGet started at {learn_url}\n",
                    game = style(&welcome_context.game.display_name).bold(),
					created_config = if welcome_context.created_config {
						format!("Created config file at {}", style("rivet.yaml").bold())
					} else {
						format!(
							"Config file already exists at {}",
							style("rivet.yaml").bold()
						)
					},
					middle = style("~ ~ ~").bold().dim(),
					learn_url = style(welcome_context.engine.learn_url())
						.italic()
						.underlined()
						.cyan()
				),
				8
			),
			width
		)
	);
	eprintln!();
}
