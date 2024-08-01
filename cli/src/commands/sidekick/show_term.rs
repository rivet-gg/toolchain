use clap::Parser;
use global_error::prelude::*;
use serde::Serialize;

use crate::util::{show_term, struct_fmt};

use super::SideKickHandler;

#[derive(Parser)]
pub struct Opts {
	#[clap(index = 1, multiple_values = true)]
	args: Vec<String>,
}

#[derive(Serialize)]
pub struct Output {
	pid: u32,
}

impl SideKickHandler for Output {}

impl Opts {
	pub async fn execute(&self) -> GlobalResult<Output> {
		let cmd = show_term::show_term(&self.args).await?;

		let output = Output { pid: cmd.id() };
		struct_fmt::print_opt(None, &output)?;
		Ok(output)
	}
}
