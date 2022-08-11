use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
pub enum SubCommand {
	Create,
	Dashboard,
}

impl SubCommand {
	pub async fn execute(&self, _ctx: &rivetctl::Ctx) -> Result<()> {
		Ok(())
	}
}
