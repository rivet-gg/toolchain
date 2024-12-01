pub mod commands;
pub mod util;

use clap::{builder::styling, Parser};
use std::process::ExitCode;

use crate::util::errors;

const STYLES: styling::Styles = styling::Styles::styled()
	.header(styling::AnsiColor::Red.on_default().bold())
	.usage(styling::AnsiColor::Red.on_default().bold())
	.literal(styling::AnsiColor::White.on_default().bold())
	.placeholder(styling::AnsiColor::White.on_default());

#[derive(Parser)]
#[clap(
	author = "Rivet Gaming, Inc. <developer@rivet.gg>",
	about = "https://rivet.gg/",
	version = concat!(env!("CARGO_PKG_VERSION"), " (", env!("VERGEN_GIT_SHA"), ")"),
	long_version = concat!(
		"\n\n",
		"git sha: ", env!("VERGEN_GIT_SHA"), "\n",
		"git branch: ", env!("VERGEN_GIT_BRANCH"), "\n",
		"build semver: ", env!("CARGO_PKG_VERSION"), "\n",
		"build timestamp: ", env!("VERGEN_BUILD_TIMESTAMP"), "\n",
		"build target: ", env!("VERGEN_CARGO_TARGET_TRIPLE"), "\n",
		"build debug: ", env!("VERGEN_CARGO_DEBUG"), "\n",
		"rustc version: ", env!("VERGEN_RUSTC_SEMVER"),
	),
    styles = STYLES
)]

struct Cli {
	#[command(subcommand)]
	command: commands::SubCommand,
}

#[tokio::main]
async fn main() -> ExitCode {
	let cli = Cli::parse();
	match cli.command.execute().await {
		Ok(()) => ExitCode::SUCCESS,
		Err(err) => {
			if err.is::<errors::GracefulExit>() {
				// Don't print anything, already handled
			} else if let Some(err) = err.downcast_ref::<errors::UserError>() {
				eprintln!("{err}");
			} else {
				eprintln!("{err}");
				// TODO: Report error
			}

			ExitCode::FAILURE
		}
	}
}
