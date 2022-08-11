use anyhow::Result;
use clap::Parser;
use commands::*;

mod commands;
mod util;

#[derive(Parser)]
#[clap()]
struct Opts {
	#[clap(subcommand)]
	command: SubCommand,

	#[clap(long, env = "RIVETCTL_API_URL")]
	api_url: Option<String>,

	#[clap(long, env = "RIVETCTL_ACCESS_TOKEN")]
	access_token: Option<String>,
}

#[derive(Parser)]
enum SubCommand {
	Create(create::Opts),
	Auth {
		#[clap(subcommand)]
		command: auth::SubCommand,
	},
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

	let ctx = rivetctl::ctx::init(
		rivetctl::config::global::read().await?,
		opts.api_url.clone(),
		opts.access_token.clone(),
	)
	.await?;

	match opts.command {
		SubCommand::Create(opts) => opts.execute(&ctx).await?,
		SubCommand::Auth { command } => command.execute(&ctx).await?,
		SubCommand::Namespace { command } => command.execute(&ctx).await?,
		SubCommand::Version { command } => command.execute(&ctx).await?,
		SubCommand::Build { command } => command.execute(&ctx).await?,
		SubCommand::Site { command } => command.execute(&ctx).await?,
	}

	Ok(())
}
