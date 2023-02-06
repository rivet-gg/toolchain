use anyhow::Result;
use clap::Parser;
use console::Term;
use tokio::fs;

use crate::util::term;

const GITHUB_WORKFLOW_RIVET_PUBLISH_YAML: &'static str =
	include_str!("../../tpl/workflows/rivet-publish.yaml");

#[derive(Parser)]
pub enum SubCommand {
	/// Creates GitHub Actions workflow file
	Init(InitOpts),
}

impl SubCommand {
	pub async fn execute(&self, term: &Term, ctx: &cli_core::Ctx) -> Result<()> {
		match self {
			SubCommand::Init(opts) => opts.execute(term, ctx).await,
		}
	}
}

#[derive(Parser)]
pub struct InitOpts {
	/// Path to the Dockerfile used to build the Serverless Lobby build
	#[clap(long = "dockerfile", alias = "dockerfile-path")]
	dockerfile_path: Option<String>,
	/// Command to run to build the CDN site
	#[clap(long = "cdn-build", alias = "cdn-build-command")]
	cdn_build_command: Option<String>,
	/// Path to directory to upload to the CDN
	#[clap(long)]
	cdn_path: Option<String>,
}

impl InitOpts {
	async fn execute(&self, term: &Term, ctx: &cli_core::Ctx) -> Result<()> {
		// Create .github/workflows/rivet-push.yaml
		let workflows_path = std::env::current_dir()?.join(".github").join("workflows");
		let actions_path = workflows_path.join("rivet-publish.yaml");
		let dockerfile_path = if let Some(x) = self.dockerfile_path.clone() {
			x
		} else {
			term::input::string(term, "Server Dockerfile path?").await?
		};
		let site_build_command = if let Some(x) = self.cdn_build_command.clone() {
			x
		} else {
			term::input::string(term, "CDN build command?").await?
		};
		let site_build_path = if let Some(x) = self.cdn_path.clone() {
			x
		} else {
			term::input::string(term, "CDN build output path?").await?
		};

		// TODO: Escape values for single quotes
		let publish_yml = GITHUB_WORKFLOW_RIVET_PUBLISH_YAML
			.replace("__DOCKERFILE_PATH__", &dockerfile_path)
			.replace("__SITE_BUILD_COMMAND__", &site_build_command)
			.replace("__SITE_BUILD_PATH__", &site_build_path);

		fs::create_dir_all(&workflows_path).await?;
		fs::write(actions_path, publish_yml).await?;

		term::status::warn(
			"Make sure to set the RIVET_CLOUD_TOKEN GitHub Actions secret",
			dashboard_api_url(&ctx.game_id),
		);

		term::status::success(
			"Finished",
			"Your game will automatically publish to Rivet next time you push to GitHub.",
		);

		Ok(())
	}
}

pub fn dashboard_api_url(game_id: &str) -> String {
	format!("https://hub.rivet.gg/developer/games/{game_id}/api")
}
