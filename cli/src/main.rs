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
	Init(init::Opts),
	Namespace {
		#[clap(subcommand)]
		command: ns::SubCommand,
	},
	Version {
		#[clap(subcommand)]
		command: version::SubCommand,
	},
	Build {
		#[clap(subcommand)]
		command: build::SubCommand,
	},
	Site {
		#[clap(subcommand)]
		command: site::SubCommand,
	},
}

#[tokio::main]
async fn main() -> Result<()> {
	let opts = Opts::parse();

	// Handle init command without the context
	if let SubCommand::Init(init_opts) = &opts.command {
		return init_opts.execute(opts.api_url).await;
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
	let ctx = rivetctl::ctx::init(opts.api_url.clone(), cloud_token).await?;

	// Handle command
	match opts.command {
		SubCommand::Init(_) => unreachable!(),
		SubCommand::Namespace { command } => command.execute(&ctx).await?,
		SubCommand::Version { command } => command.execute(&ctx).await?,
		SubCommand::Build { command } => command.execute(&ctx).await?,
		SubCommand::Site { command } => command.execute(&ctx).await?,
	}

	Ok(())
}
