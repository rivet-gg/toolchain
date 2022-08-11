use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
pub struct Opts {
	#[clap(index(1))]
	name_id: String,
}

impl Opts {
	pub async fn execute(&self, ctx: &rivetctl::Ctx) -> Result<()> {
		Ok(())
	}
}
