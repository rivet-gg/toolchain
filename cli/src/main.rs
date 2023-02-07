use anyhow::{Context, Result};
use clap::Parser;
use commands::*;
use util::secrets;

mod commands;
mod util;

#[derive(Parser)]
#[clap(
	author = "Rivet Gaming, Inc. <developer@rivet.gg>",
	about = "https://rivet.gg/",
	version = env!("VERGEN_GIT_SHA"),
	long_version = concat!(
		"\n",
		"git sha: ", env!("VERGEN_GIT_SHA"), "\n",
		"git branch: ", env!("VERGEN_GIT_BRANCH"), "\n",
		"build timestamp: ", env!("VERGEN_BUILD_TIMESTAMP"), "\n",
		"build target: ", env!("VERGEN_CARGO_TARGET_TRIPLE"), "\n",
		"build profile: ", env!("VERGEN_CARGO_PROFILE"), "\n",
		"rustc version: ", env!("VERGEN_RUSTC_SEMVER"),
	)
)]
struct Opts {
	#[clap(subcommand)]
	command: SubCommand,

	#[clap(long, env = "RIVET_CLOUD_API_URL")]
	api_url: Option<String>,

	#[clap(long, env = "RIVET_CLOUD_TOKEN")]
	cloud_token: Option<String>,
}

#[derive(Parser)]
enum SubCommand {
	/// Guided setup for this project
	Init(init::Opts),

	/// Opens the dashboard for this game
	#[clap(alias = "dash")]
	Dashboard,

	/// Initiates the development environment for this project
	Dev {
		#[clap(subcommand)]
		command: dev::SubCommand,
	},

	/// Manages the game
	Game {
		#[clap(subcommand)]
		command: game::SubCommand,
	},

	/// Manages namespaces
	#[clap(alias = "ns")]
	Namespace {
		#[clap(subcommand)]
		command: ns::SubCommand,
	},

	/// Manages versions
	Version {
		#[clap(subcommand)]
		command: version::SubCommand,
	},

	/// Manages builds for Serverless Lobbies
	#[clap(alias = "build")]
	Image {
		#[clap(subcommand)]
		command: image::SubCommand,
	},

	/// Manages sites for the CDN
	Site {
		#[clap(subcommand)]
		command: site::SubCommand,
	},

	/// Manages identity avatars
	IdentityAvatar {
		#[clap(subcommand)]
		command: avatar::SubCommand,
	},

	/// Alias of `rivet version publish`
	#[clap(alias = "deploy")]
	Publish(version::PublishOpts),
}

#[tokio::main]
async fn main() -> Result<()> {
	let term = console::Term::stderr();
	let opts = Opts::parse();

	// Handle init command without the context
	if let SubCommand::Init(init_opts) = &opts.command {
		return init_opts
			.execute(
				opts.cloud_token.as_ref().map(String::as_str),
				&term,
				opts.api_url,
			)
			.await;
	}

	// Read cloud token
	let cloud_token = if let Some(cloud_token) = opts.cloud_token {
		cloud_token
	} else {
		secrets::read_cloud_token()
			.await?
			.context("no Rivet cloud token found")?
	};

	// Create context
	let ctx = cli_core::ctx::init(opts.api_url.clone(), cloud_token).await?;

	// Handle command
	match opts.command {
		SubCommand::Init(_) => unreachable!(),
		SubCommand::Dashboard => {
			println!(
				"https://hub.rivet.gg/developer/games/{game_id}",
				game_id = ctx.game_id
			);
		}
		SubCommand::IdentityAvatar { command } => command.execute(&ctx).await?,
		SubCommand::Dev { command } => command.execute(&term, &ctx).await?,
		SubCommand::Game { command } => command.execute(&ctx).await?,
		SubCommand::Namespace { command } => command.execute(&ctx).await?,
		SubCommand::Version { command } => command.execute(&ctx).await?,
		SubCommand::Image { command } => command.execute(&ctx).await?,
		SubCommand::Site { command } => command.execute(&ctx).await?,
		SubCommand::Publish(opts) => opts.execute(&ctx).await?,
	}

	Ok(())
}
