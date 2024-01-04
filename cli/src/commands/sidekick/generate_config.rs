use clap::Parser;
use global_error::prelude::*;
use serde::Serialize;
use tokio::fs;

use crate::util::version_config::Engine;

use super::SideKickHandler;

#[derive(Parser)]
pub struct Opts {
	#[clap(long)]
	unity: bool,
	#[clap(long)]
	unreal: bool,
	#[clap(long)]
	godot: bool,
	#[clap(long)]
	html5: bool,
	#[clap(long)]
	custom: bool,
}

#[derive(Serialize)]
pub struct Output {
	pub output: String,
}

impl SideKickHandler for Output {}

impl Opts {
	pub async fn execute(&self) -> GlobalResult<Output> {
		let engine = if self.unity {
			Engine::Unity
		} else if self.unreal {
			Engine::Unreal
		} else if self.godot {
			Engine::Godot
		} else if self.html5 {
			Engine::Html5
		} else {
			Engine::Custom
		};

		let current_dir = std::env::current_dir()?;
		let config_exists = ["rivet.yaml", "rivet.toml", "rivet.json"]
			.iter()
			.any(|file_name| current_dir.join(file_name).exists());

		if !config_exists {
			// Write file
			let version_config = crate::util::version_config::generate(&engine, true)?;
			fs::write(current_dir.join("rivet.yaml"), version_config).await?;

			return Ok(Output {
				output: "Created rivet.yaml".to_string(),
			});
		} else {
			return Ok(Output {
				output:
					"Version already configured. Your game is already configured with rivet.yaml"
						.to_string(),
			});
		};
	}
}
