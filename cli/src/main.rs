use anyhow::{Context, Result};
use clap::Parser;
use commands::*;
use util::secrets;

mod commands;
mod util;

#[derive(Parser)]
#[clap()]
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

	/// Helper functions for continuous integration
	CI {
		#[clap(subcommand)]
		command: ci::SubCommand,
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
	Build {
		#[clap(subcommand)]
		command: build::SubCommand,
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
		SubCommand::CI { command } => command.execute(&term, &ctx).await?,
		SubCommand::Game { command } => command.execute(&ctx).await?,
		SubCommand::Namespace { command } => command.execute(&ctx).await?,
		SubCommand::Version { command } => command.execute(&ctx).await?,
		SubCommand::Build { command } => command.execute(&ctx).await?,
		SubCommand::Site { command } => command.execute(&ctx).await?,
	}

	Ok(())
}
