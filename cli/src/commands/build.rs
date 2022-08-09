use clap::Parser;

#[derive(Parser)]
pub enum SubCommand {
	Push(BuildPushOpts),
}

#[derive(Parser)]
pub struct BuildPushOpts {
	#[clap(index(1))]
	pub tag: String,

	#[clap(long)]
	pub name: Option<String>,
}
