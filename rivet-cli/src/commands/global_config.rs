use clap::Parser;
use global_error::prelude::*;

use crate::util::{global_config, paths, struct_fmt};

#[derive(Parser)]
pub enum SubCommand {
	/// Reads the project's config
	#[clap()]
	ReadProject {
		#[clap(long, value_parser)]
		format: struct_fmt::Format,
	},
	/// Prints the path to the global config
	#[clap()]
	Path,
}

impl SubCommand {
	pub async fn execute(&self) -> GlobalResult<()> {
		match self {
			SubCommand::ReadProject { format } => {
				global_config::read_project(|project| struct_fmt::print(format, project)).await??;

				Ok(())
			}
			SubCommand::Path => {
				print!("{}", paths::global_config_file()?.display());

				Ok(())
			}
		}
	}
}
