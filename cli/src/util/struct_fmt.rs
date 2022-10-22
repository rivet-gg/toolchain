use anyhow::Result;
use clap::ValueEnum;
use serde::Serialize;

#[derive(ValueEnum, Clone)]
pub enum Format {
	Json,
}

pub fn print(format: &Format, data: &impl Serialize) -> Result<()> {
	match format {
		Format::Json => {
			let output = serde_json::to_string(data)?;
			println!("{output}");
		}
	}

	Ok(())
}

pub fn print_opt(format: Option<&Format>, data: &impl Serialize) -> Result<()> {
	if let Some(format) = format.as_ref() {
		print(format, data)?;
	}

	Ok(())
}
