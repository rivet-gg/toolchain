use anyhow::{bail, Context, Result};
use clap::Parser;
use cli_core::rivet_api::models;
use console::Term;
use serde::Serialize;
use std::collections::HashMap;
use tokio::fs;

use crate::{
	commands,
	util::{struct_fmt, term},
};

#[derive(Parser)]
pub enum SubCommand {
	CreateDevToken(crate::commands::token::create::dev::Opts),
}

impl SubCommand {
	pub async fn execute(&self, term: &Term, ctx: &cli_core::Ctx) -> Result<()> {
		match self {
			SubCommand::CreateDevToken(opts) => {
				term::status::warn(
					"This command is deprecated. ",
					"Please use `rivet token create dev` instead.",
				);

				opts.execute(term, ctx).await
			}
		}
	}
}
