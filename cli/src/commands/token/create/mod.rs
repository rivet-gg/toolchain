use anyhow::{Context, Result};
use clap::Parser;
use console::Term;
use serde::Serialize;

use crate::util::{struct_fmt, term};

pub mod dev;

#[derive(Parser)]
pub enum SubCommand {
	#[clap(alias = "dev")]
	Development(dev::Opts),
}

impl SubCommand {
	pub async fn execute(&self, term: &Term, ctx: &cli_core::Ctx) -> Result<()> {
		match self {
			SubCommand::Development(opts) => opts.execute(term, ctx).await,
		}
	}
}

