use anyhow::{Context, Result};
use clap::Parser;
use console::Term;
use tokio::process::Command;

use crate::commands;

#[derive(Parser)]
pub enum SubCommand {
	/// Starts a server locally
	StartServer,
}

impl SubCommand {
	pub async fn execute(&self, term: &Term, ctx: &cli_core::Ctx) -> Result<()> {
		match self {
			SubCommand::StartServer => {
				let pwd = std::env::current_dir()?;

				let token = commands::token::create::dev::execute(
					term,
					ctx,
					&commands::token::create::dev::Opts {
						dev_env: Some(false),
						namespace: None,
						format: None,
					},
				)
				.await?
				.token;

				// Build base image
				Command::new("docker")
					.args(&[
						"build",
						"-f",
						"server.development.Dockerfile",
						"-t",
						"rivet-unreal-server-development",
						".",
					])
					.spawn()?
					.wait()
					.await?;

				// Run container
				Command::new("docker")
					.args(&[
						"run",
						"-it",
						"--rm",
						"--env",
						&format!("RIVET_TOKEN={}", token),
						"-v",
						&format!("{}:/project", pwd.display()),
						"-p",
						"127.0.0.1:7777:7777/udp",
						"rivet-unreal-server-development",
					])
					.spawn()?
					.wait()
					.await?;

				Ok(())
			}
		}
	}
}
