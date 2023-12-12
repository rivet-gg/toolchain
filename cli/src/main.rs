use crate::util::global_config;
use anyhow::{Context, Result};
use clap::Parser;
use commands::*;

mod commands;
mod util;

// IMPORTANT: Do not read `api_endpoint`, `token`, and `telemetry_disabled` directly from `opts`. These properties are written to the config. Use `global_config::read_project`.
#[derive(Parser)]
#[clap(
	author = "Rivet Gaming, Inc. <developer@rivet.gg>",
	about = "https://rivet.gg/",
	version = concat!(env!("VERGEN_BUILD_SEMVER"), " (", env!("VERGEN_GIT_SHA_SHORT"), ")"),
	long_version = concat!(
		"\n\n",
		"git sha: ", env!("VERGEN_GIT_SHA"), "\n",
		"git branch: ", env!("VERGEN_GIT_BRANCH"), "\n",
		"build semver: ", env!("VERGEN_BUILD_SEMVER"), "\n",
		"build timestamp: ", env!("VERGEN_BUILD_TIMESTAMP"), "\n",
		"build target: ", env!("VERGEN_CARGO_TARGET_TRIPLE"), "\n",
		"build profile: ", env!("VERGEN_CARGO_PROFILE"), "\n",
		"rustc version: ", env!("VERGEN_RUSTC_SEMVER"),
	)
)]
struct Opts {
	#[clap(subcommand)]
	command: SubCommand,

	#[clap(global = true, long, env = "RIVET_API_ENDPOINT")]
	api_endpoint: Option<String>,

	#[clap(global = true, long, env = "RIVET_TOKEN")]
	token: Option<String>,

	#[clap(global = true, long, env = "TELEMETRY_DISABLED")]
	telemetry_disabled: Option<bool>,
}

#[derive(Parser)]
enum SubCommand {
	/// Guided setup for this project
	Init(init::Opts),

	/// Opens the dashboard for this game
	#[clap(alias = "dash")]
	Dashboard,

	/// Manages tokens
	Token {
		#[clap(subcommand)]
		command: token::SubCommand,
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
	#[clap(hide = true)]
	IdentityAvatar {
		#[clap(subcommand)]
		command: avatar::SubCommand,
	},

	/// Alias of `rivet version deploy`
	#[clap(alias = "publish")]
	Deploy(version::DeployOpts),

	/// Run engine-specific commands
	Engine {
		#[clap(subcommand)]
		command: engine::SubCommand,
	},

	/// Alias of `rivet engine unreal`
	#[clap(hide = true, alias = "ue")]
	Unreal {
		#[clap(subcommand)]
		command: engine::unreal::SubCommand,
	},

	/// Deprecated.
	///
	/// Initiates the development environment for this project.
	#[clap(hide = true)]
	Dev {
		#[clap(subcommand)]
		command: dev::SubCommand,
	},
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
	let opts = read_opts().await?;

	let res = main_inner(opts).await;

	// Blanket catch for all errors
	if let Err(err) = &res {
		util::telemetry::capture_event(
			util::telemetry::GAME_ID.get(),
			"cli_error",
			Some(|event: &mut async_posthog::Event| {
				event.insert_prop(
					"errors",
					err.chain().map(|e| e.to_string()).collect::<Vec<_>>(),
				)?;
				Ok(())
			}),
		)
		.await?;
	}

	util::telemetry::wait_all().await;

	res
}

async fn main_inner(opts: Opts) -> Result<()> {
	let term = console::Term::stderr();

	// Handle init command without the context
	if let SubCommand::Init(init_opts) = &opts.command {
		return init_opts.execute(&term).await;
	}

	// Read token
	let (api_endpoint, token) =
		global_config::read_project(|x| (x.cluster.api_endpoint.clone(), x.tokens.cloud.clone()))
			.await?;
	let token = token.context("no Rivet token found, please run `rivet init`")?;
	let ctx = cli_core::ctx::init(api_endpoint, token).await?;

	// Set game id for errors
	util::telemetry::GAME_ID.set(ctx.game_id.clone())?;

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
		SubCommand::Dev { command } => command.execute(&ctx).await?,
		SubCommand::Token { command } => command.execute(&ctx).await?,
		SubCommand::Game { command } => command.execute(&ctx).await?,
		SubCommand::Namespace { command } => command.execute(&ctx).await?,
		SubCommand::Version { command } => command.execute(&ctx).await?,
		SubCommand::Image { command } => command.execute(&ctx).await?,
		SubCommand::Site { command } => command.execute(&ctx).await?,
		SubCommand::Deploy(opts) => opts.execute(&ctx).await?,
		SubCommand::Engine { command } => command.execute(&ctx).await?,
		SubCommand::Unreal { command } => command.execute(&ctx).await?,
	}

	Ok(())
}

/// Reads options from clap and reads/updates the internal config file.
async fn read_opts() -> Result<Opts> {
	let opts = Opts::parse();

	global_config::mutate_project(|config| {
		if let Some(api_endpoint) = &opts.api_endpoint {
			config.cluster.api_endpoint = Some(api_endpoint.clone());
		}

		if let Some(token) = &opts.token {
			config.tokens.cloud = Some(token.clone());
		}

		if let Some(telemetry_disabled) = opts.telemetry_disabled {
			config.telemetry.disabled = telemetry_disabled;
		}
	})
	.await?;

	Ok(opts)
}
