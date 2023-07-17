use anyhow::{Context, Result};
use clap::Parser;
use console::Term;
use serde::Serialize;

use crate::util::{struct_fmt, term};

pub mod create;

#[derive(Parser)]
pub enum SubCommand {
	#[clap()]
	Create { #[clap(subcommand)] command: create::SubCommand},
}

impl SubCommand {
	pub async fn execute(&self, term: &Term, ctx: &cli_core::Ctx) -> Result<()> {
		match self {
			SubCommand::Create{command} => command.execute(term, ctx).await,
		}
	}
}

