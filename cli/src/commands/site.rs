use clap::Parser;

#[derive(Parser)]
pub enum SubCommand {
	Push(SitePushOptions),
}

#[derive(Parser)]
pub struct SitePushOptions {
	#[clap(index(1))]
	pub path: String,

	#[clap(long)]
	pub name: Option<String>,
}
