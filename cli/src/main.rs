use anyhow::{bail, Context, Result};
use clap::Parser;
use commands::*;
use futures_util::stream::{StreamExt, TryStreamExt};
use rand::{thread_rng, Rng};
use std::{
	env,
	path::{Path, PathBuf},
	sync::{
		atomic::{AtomicU64, AtomicUsize, Ordering},
		Arc,
	},
	time::{Duration, Instant},
};
use tokio::fs;

mod commands;
mod util;

#[derive(Parser)]
#[clap()]
struct Opts {
	#[clap(subcommand)]
	command: SubCommand,

	#[clap(long, env = "RIVET_CLOUD_API_URL")]
	api_url: Option<String>,

	#[clap(long, env = "RIVET_CLOUD_ACCESS_TOKEN")]
	access_token: Option<String>,
}

#[derive(Parser)]
enum SubCommand {
	Auth {
		#[clap(subcommand)]
		command: auth::SubCommand,
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
		SubCommand::Auth { command } => command.execute(&ctx).await?,
		SubCommand::Build { command } => command.execute(&ctx).await?,
		SubCommand::Site { command } => command.execute(&ctx).await?,
	}

	Ok(())
}
