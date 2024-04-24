use clap::Parser;
use console::Term;
use global_error::prelude::*;
use tokio::process::Command;

use crate::util::term;

pub mod deploy;
pub mod environment;

#[derive(Parser)]
pub enum SubCommand {
	Deploy(deploy::Opts),

	#[clap(alias = "env")]
	Environment {
		#[clap(subcommand)]
		command: environment::SubCommand,
	},

	// NOTE: This isn't an actual command, it is just added for the help entry
	/// Passthrough to the OpenGB CLI
	#[clap(name = "<any OpenGB CLI command>")]
	Any,
}

impl SubCommand {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> GlobalResult<()> {
		match self {
			SubCommand::Deploy(opts) => opts.execute(&ctx).await,
			SubCommand::Environment { command } => command.execute(&ctx).await,
			SubCommand::Any => unreachable!(),
		}
	}

	pub async fn passthrough(term: &Term) -> GlobalResult<()> {
		let mut cmd = Command::new("opengb");
		let installed = cmd.output().await?.status.success();

		if !installed {
			// Prompt for OpenGB CLI install
			let install = rivet_term::prompt::PromptBuilder::default()
				.message(
					"The OpenGB CLI `opengb` is not installed. Would you like to install it now?",
				)
				.docs_url("https://github.com/rivet-gg/opengb")
				.build()?
				.bool(term)
				.await?;

			ensure!(
				install,
				"OpenGB CLI is required to use the `backend` passthrough command."
			);

			// Check if deno is installed
			let mut cmd = Command::new("deno");
			cmd.arg("--version");

			let installed = cmd.output().await?.status.success();
			ensure!(
				installed,
				"The Deno CLI tool `deno` is not installed. Install it from {}.",
				term::link("https://docs.deno.com/runtime/manual"),
			);

			// Install OpenGB CLI
			let mut cmd = Command::new("deno");
			cmd.arg("install");
			cmd.arg("--allow-net");
			cmd.arg("--allow-read");
			cmd.arg("--allow-env");
			cmd.arg("--allow-run");
			cmd.arg("--allow-write");
			cmd.arg("--name").arg("opengb");
			cmd.arg("--force");
			cmd.arg("https://raw.githubusercontent.com/rivet-gg/opengb/3aab9bc2abcb8105fc3af837900ce4f7a932ad17/src/cli/main.ts");

			ensure!(
				cmd.status().await?.success(),
				"failed to install OpenGB CLI"
			);
		}

		let mut cmd = Command::new("opengb");

		for arg in std::env::args().skip(2) {
			cmd.arg(arg);
		}

		// TODO: How does this play with the sentry task?
		// Match the exit code of the opengb command
		if let Some(exit_code) = cmd.status().await?.code() {
			std::process::exit(exit_code);
		}

		Ok(())
	}
}
