use clap::ValueEnum;
use global_error::prelude::*;
use serde::Serialize;

#[derive(ValueEnum, Clone)]
pub enum Format {
	Json,
	JsonCompact,
}

pub fn print(format: &Format, data: &impl Serialize) -> GlobalResult<()> {
	match format {
		Format::Json => {
			let output = serde_json::to_string_pretty(data)?;
			println!("{output}");
		}
		Format::JsonCompact => {
			let output = serde_json::to_string(data)?;
			println!("{output}");
		}
	}

	Ok(())
}

pub fn print_opt(format: Option<&Format>, data: &impl Serialize) -> GlobalResult<()> {
	if let Some(format) = format.as_ref() {
		print(format, data)?;
	}

	Ok(())
}
