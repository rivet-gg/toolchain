use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
pub enum SubCommand {
	Create,
	SetVersion,
	Dashboard,
	Visit,
}

impl SubCommand {
	pub async fn execute(&self, ctx: &rivetctl::Ctx) -> Result<()> {
		Ok(())
	}
}
