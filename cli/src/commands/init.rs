use anyhow::{bail, ensure, Context, Result};
use clap::Parser;
use cli_core::{ctx, rivet_api, Ctx};
use console::{style, Term};
use std::path::{Path, PathBuf};
use tokio::{fs, io::AsyncWriteExt};

use crate::{
	commands,
	util::{git, secrets, term},
};

const CONFIG_DEFAULT_HEAD: &'static str = include_str!("../../tpl/default_config/head.toml");
const CONFIG_DEFAULT_CDN: &'static str = include_str!("../../tpl/default_config/cdn.toml");
const CONFIG_DEFAULT_MM: &'static str = include_str!("../../tpl/default_config/matchmaker.toml");

const CONFIG_UNREAL: &'static str = include_str!("../../tpl/unreal_config/head.toml");

#[derive(Parser)]
pub struct Opts {
	#[clap(long)]
	recommend: bool,
	#[clap(long)]
	update_gitignore: bool,
	#[clap(long)]
	create_version_config: bool,

	// Presets
	#[clap(long, alias = "unreal-engine")]
	unreal: bool,

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
	pub async fn execute(
		&self,
		cloud_token: Option<&str>,
		term: &Term,
		override_api_url: Option<String>,
	) -> Result<()> {
		let ctx = self.build_ctx(term, cloud_token, override_api_url).await?;

		self.update_gitignore(term, &ctx).await?;

		if self.unreal {
			self.create_config_unreal(term, &ctx).await?;
		} else {
			// Default pipeline
			let has_version_config = self.create_config_default(term, &ctx).await?;
			self.create_dev_token(term, &ctx, has_version_config)
				.await?;
		}

		eprintln!();
		term::status::success(
			"What's next?",
			"https://docs.rivet.gg/general/guides/crash-course",
		);

		Ok(())
	}

	async fn build_ctx(
		&self,
		term: &Term,
		cloud_token: Option<&str>,
		override_api_url: Option<String>,
	) -> Result<Ctx> {
		// Check if token already exists
		let cloud_token = if let Some(cloud_token) = cloud_token.clone() {
			Some(cloud_token.to_string())
		} else {
			secrets::read_cloud_token().await?
		};
		let ctx = if let Some(cloud_token) = cloud_token {
			let ctx = cli_core::ctx::init(override_api_url.clone(), cloud_token).await?;

			let game_res = ctx
				.client()
				.get_game_by_id()
				.game_id(&ctx.game_id)
				.send()
				.await
				.context("client.get_game_by_id")?;
			let game = game_res.game().context("game_res.game")?;
			let display_name = game.display_name().context("game.display_name")?;

			term::status::success("Found existing token", display_name);

			ctx
		} else {
			read_cloud_token(term, override_api_url.clone()).await?
		};

		Ok(ctx)
	}

	async fn update_gitignore(&self, term: &Term, ctx: &Ctx) -> Result<()> {
		if !git::check_ignore(Path::new(".rivet/")).await? {
			if self.recommend
				|| self.update_gitignore
				|| term::Prompt::new("Add .rivet/ to .gitignore?")
					.docs(".rivet/ holds secrets and local configuration files that should not be version controlled")
					.docs_url("https://docs.rivet.gg/general/concepts/dot-rivet-directory")
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

	async fn create_config_unreal(&self, term: &Term, ctx: &Ctx) -> Result<()> {
		let config_path = std::env::current_dir()?.join("rivet.toml");
		let config_needs_creation = match fs::read_to_string(&config_path).await {
			Ok(_) => false,
			Err(err) if err.kind() == std::io::ErrorKind::NotFound => true,
			Err(err) => {
				return Err(err.into());
			}
		};

		// Read module name
		let mut module_name_prompt = term::Prompt::new("Unreal game module name?").docs("Name of the Unreal module that holds the game code. This is usually the value of `$.Modules[0].Name` in the file `MyProject.unproject`.");
		if let Some(module_name) = attempt_read_module_name().await? {
			module_name_prompt = module_name_prompt.default_value(module_name);
		}
		let module_name = module_name_prompt.string(term).await?;

		// Generate config file
		if config_needs_creation || self.create_version_config {
			let version_config = CONFIG_UNREAL.replace("__GAME_MODULE__", &module_name);

			fs::write(config_path, version_config).await?;

			term::status::success("Created rivet.toml", "");
		} else {
			term::status::success(
				"Version already configured",
				"Your game is already configured with rivet.toml",
			);
		}

		// Generate Dockerfile

		Ok(())
	}

	async fn create_config_default(&self, term: &Term, ctx: &Ctx) -> Result<bool> {
		let config_path = std::env::current_dir()?.join("rivet.toml");
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
				|| term::Prompt::new("Create rivet.toml?")
					.docs("This is the configuration file used to manage your game")
					.docs_url("https://docs.rivet.gg/general/concepts/rivet-version-config")
					.default_value("yes")
					.bool(term)
					.await?
			{
				let mut version_config = CONFIG_DEFAULT_HEAD.to_string();

				if self.matchmaker
					|| term::Prompt::new("Enable Rivet Matchmaker?")
						.indent(1)
						.context("rivet.toml")
						.docs("Setup your matchmaker configuration, this can be changed later")
						.docs_url("https://docs.rivet.gg/matchmaker/introduction")
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
						.context("rivet.toml")
						.docs("Setup service a website or static assets, this can be changed later")
						.docs_url("https://docs.rivet.gg/cdn/introduction")
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

				term::status::success("Created rivet.toml", "");

				true
			} else {
				false
			}
		} else {
			term::status::success(
				"Version already configured",
				"Your game is already configured with rivet.toml",
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
			.docs_url("http://docs.rivet.gg/general/concepts/dev-tokens")
			.bool(term)
			.await?)
		{
			commands::dev::CreateDevTokenOpts {
				dev_env: self.recommend || self.dev_env,
				format: None,
			}
			.execute(term, &ctx)
			.await?
		}

		Ok(())
	}
}

async fn read_cloud_token(term: &Term, override_api_url: Option<String>) -> Result<cli_core::Ctx> {
	// Create OpenAPI configuration without bearer token to send link request
	let openapi_config_cloud_unauthed = rivet_api::apis::configuration::Configuration {
		base_path: override_api_url
			.clone()
			.unwrap_or_else(|| ctx::DEFAULT_API_CLOUD_URL.to_string()),
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
	let cloud_token = loop {
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

		if let Some(cloud_token) = prepare_res.cloud_token {
			break cloud_token;
		}
	};

	// Create new context
	let new_ctx = cli_core::ctx::init(
		override_api_url,
		// Exclude overridden access token to check the token
		cloud_token.clone(),
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
	secrets::write_cloud_token(&cloud_token).await?;

	term::status::success("Token Saved", display_name);

	Ok(new_ctx)
}

/// Finds the Unreal project file in the current directory.
async fn find_uproject_file() -> Result<Option<PathBuf>> {
	let current_dir = std::env::current_dir().ok()?;
	let mut read_dir = fs::read_dir(current_dir).await.ok()?;
	while let Some(entry) = read_dir.next_entry().await.ok()?.flatten() {
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
async fn attempt_read_module_name() -> Result<Option<String>> {
	// Read uproject file
	let uproject_path =
		if let Some(path) = find_uproject_file().await.context("find_uproject_file")? {
			path
		} else {
			return Ok(None);
		};
	let uproject_str = match fs::read_to_string(&uproject_path).await {
		Ok(uproject) => uproject,
		Err(err) => {
			return Ok(None);
		}
	};
	let uproject_json = match serde_json::from_str::<serde_json::Value>(&uproject_str) {
		Ok(uproject_json) => uproject_json,
		Err(err) => {
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
