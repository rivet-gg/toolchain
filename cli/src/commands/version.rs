use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
pub enum SubCommand {
	List,
	Create,
	Dashboard,
}

impl SubCommand {
	pub async fn execute(&self, ctx: &rivetctl::Ctx) -> Result<()> {
		match self {
			SubCommand::List => todo!(),
			SubCommand::Create => {
				todo!()
			}
			SubCommand::Dashboard => {
				todo!()
			}
		}
	}
}
