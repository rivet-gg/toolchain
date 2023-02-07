use anyhow::{bail, ensure, Context, Result};
use clap::Parser;
use console::Term;
use std::path::Path;
use tokio::{fs, io::AsyncWriteExt};

use crate::util::{git, secrets, term};

const VERSION_HEAD: &'static str = include_str!("../../tpl/default_config/head.toml");
const VERSION_CDN: &'static str = include_str!("../../tpl/default_config/cdn.toml");
const VERSION_MATCHMAKER: &'static str = include_str!("../../tpl/default_config/matchmaker.toml");
const VERSION_FOOT: &'static str = include_str!("../../tpl/default_config/foot.toml");

#[derive(Parser)]
pub struct Opts {
	#[clap(flatten)]
	dev_opts: crate::commands::dev::CreateDevTokenOpts,
}

impl Opts {
	pub async fn execute(
		&self,
		cloud_token: Option<&str>,
		term: &Term,
		override_api_url: Option<String>,
	) -> Result<()> {
		// Check if token already exists
		eprintln!();
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

		// Update .gitignore
		eprintln!();
		if !git::check_ignore(Path::new(".rivet/")).await? {
			if 
				term::input::bool_with_docs(
					term,
					"Add .rivet/ to .gitignore?",
					".rivet/ holds secrets and local configuration files that should not be version controlled",
					"https://docs.rivet.gg/general/concepts/dot-rivet-directory",
				)
				.await?
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

		// Create rivet.version.toml
		eprintln!();
		let config_path = std::env::current_dir()?.join("rivet.version.toml");
		let config_needs_creation = match fs::read_to_string(&config_path).await {
			Ok(_) => false,
			Err(err) if err.kind() == std::io::ErrorKind::NotFound => true,
			Err(err) => {
				return Err(err.into());
			}
		};
		let has_version_config = if config_needs_creation {
			if term::input::bool_with_docs(
					term,
					"Create rivet.version.toml?",
					"This is the configuration file used to manage your game",
					"https://docs.rivet.gg/general/concepts/rivet-version-config",
				)
				.await?
			{
				let mut version_config = VERSION_HEAD.to_string();

				eprintln!();
				if term::input::bool_with_docs(
						term,
						"rivet.version.toml > Enable Rivet Matchmaker?",
						"Setup your matchmaker configuration, this can be changed later",
						"https://docs.rivet.gg/matchmaker/introduction",
					)
					.await?
				{
					let port = 'port: loop {
						let mut port = term::input::string_with_tip(
							term,
							"rivet.version.toml > Matchmaker > What port does your game socket listen on?",
							"default: 8080",
						)
						.await?;
						if port.is_empty() {
							port = "8080".to_string();
						}
						if let Ok(port) = port.parse::<u16>() {
							break 'port port;
						} else {
							term::status::error("Invalid port number", "");
							eprintln!();
						}
					};

					let mut dockerfile_path = term::input::string_with_tip(
						term,
						"rivet.version.toml > Matchmaker > Path to the server's Dockerfile?",
						"default: Dockerfile",
					)
					.await?;
					if dockerfile_path.is_empty() {
						dockerfile_path = "Dockerfile".to_string();
					}

					version_config.push_str(
						&VERSION_MATCHMAKER
							.replace("__DOCKERFILE__", &dockerfile_path)
							.replace("__PORT__", &port.to_string()),
					);
				}

				eprintln!();
				if term::input::bool_with_docs(
						term,
						"rivet.version.toml > Enable Rivet CDN?",
						"Setup service a website or static assets, this can be changed later",
						"https://docs.rivet.gg/cdn/introduction",
					)
					.await?
				{
					let mut build_command = term::input::string_with_tip(
						term,
						"rivet.version.toml > CDN > What command will run before uploading your site?",
						"default: echo 'Nothing to do'",
					)
					.await?;
					if build_command.is_empty() {
						build_command = "echo 'Nothing to do'".to_string();
					}

					let mut build_output = term::input::string_with_tip(
						term,
						"rivet.version.toml > CDN > What directory should be uploaded to Rivet CDN?",
						"default: dist/",
					)
					.await?;
					if build_output.is_empty() {
						build_output = "dist/".to_string();
					}

					version_config.push_str(&
						VERSION_CDN
							.replace("__BUILD_COMMAND__", &build_command.replace("\"", "\\\""))
							.replace("__BUILD_OUTPUT__", &build_output)
					);
				}

				version_config.push_str(&VERSION_FOOT);

				fs::write(config_path, version_config).await?;

				term::status::success("Created rivet.version.toml", "");

				true
			} else {
				false
			}
		} else {
			term::status::success(
				"Version already configured",
				"Your game is already configured with rivet.version.toml",
			);
			true
		};

		// Development flow
		eprintln!();
		if has_version_config && term::input::bool_with_docs(
			term,
			"Setup development environment?",
			"Create development tokens that enable you to develop your game locally",
			"http://docs.rivet.gg/general/concepts/dev-tokens",
		)
		.await?
		{
			self.dev_opts.execute(term, &ctx).await?
		}

		eprintln!();
		term::status::success(
			"What's next?",
			"https://docs.rivet.gg/general/guides/crash-course",
		);

		Ok(())
	}
}

async fn read_cloud_token(term: &Term, override_api_url: Option<String>) -> Result<cli_core::Ctx> {
	let token = term::input::secure_with_docs(
		term,
		"Rivet cloud token?",
		"Create this token under Developer > My Game > API > Create Cloud Token",
		"https://docs.rivet.gg/general/concepts/tokens#cloud",
	)
	.await?;

	// Create new context
	let new_ctx = cli_core::ctx::init(
		override_api_url,
		// Exclude overridden access token to check the token
		token.clone(),
	)
	.await?;
	let inspect = new_ctx
		.client()
		.inspect()
		.send()
		.await
		.context("client.inspect()")?;

	let game_id = match inspect.agent.as_ref().context("inspect.agent")? {
		cli_core::rivet_cloud::model::AuthAgent::GameCloud(game_cloud) => {
			game_cloud.game_id.clone().context("game_cloud.game_id")?
		}
		_ => bail!("invalid agent kind"),
	};

	let game_res = new_ctx
		.client()
		.get_game_by_id()
		.game_id(game_id)
		.send()
		.await
		.context("client.get_game_by_id()")?;
	let game = game_res.game().context("game_res.game")?;
	let display_name = game.display_name().context("game.display_name")?;

	// Write the token
	secrets::write_cloud_token(&token).await?;

	term::status::success("Token Saved", display_name);

	Ok(new_ctx)
}

pub fn dashboard_api_url(game_id: &str) -> String {
	format!("https://hub.rivet.gg/developer/games/{game_id}/api")
}
