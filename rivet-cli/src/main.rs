pub mod commands;

use clap::Parser;
use global_error::GlobalResult;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: commands::SubCommand,
}

#[tokio::main]
async fn main() -> GlobalResult<()> {
	let cli = Cli::parse();
	cli.command.execute().await?;
	Ok(())
}

