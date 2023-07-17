use anyhow::Result;
use clap::Parser;
use tokio::process::Command;

#[derive(Parser)]
pub enum SubCommand {
	/// Starts a server locally
	StartServer,
}

impl SubCommand {
	pub async fn execute(&self, _ctx: &cli_core::Ctx) -> Result<()> {
		match self {
			SubCommand::StartServer => {
                let pwd = std::env::current_dir()?;

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
