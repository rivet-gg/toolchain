pub mod commands;

use clap::Parser;
use std::process::ExitCode;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: commands::SubCommand,
}

#[tokio::main]
async fn main() -> ExitCode {
	let cli = Cli::parse();
	cli.command.execute().await
}
