use clap::Parser;
use global_error::prelude::*;
use serde::Serialize;
use std::fs;

use crate::commands::init::InitEngine;

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

const CONFIG_DEFAULT_HEAD: &'static str = include_str!("../../../tpl/default_config/head.yaml");

impl Opts {
	pub fn execute(&self) -> GlobalResult<Output> {
		let engine = if self.unity {
			InitEngine::Unity
		} else if self.unreal {
			InitEngine::Unreal
		} else if self.godot {
			InitEngine::Godot
		} else if self.html5 {
			InitEngine::HTML5
		} else {
			InitEngine::Custom
		};

		let current_dir = std::env::current_dir()?;
		let config_exists = ["rivet.yaml", "rivet.toml", "rivet.json"]
			.iter()
			.any(|file_name| current_dir.join(file_name).exists());

		if !config_exists {
			let mut version_config =
				CONFIG_DEFAULT_HEAD.replace("__LEARN_URL__", &engine.learn_url());

			// Add engine config
			match engine {
				InitEngine::Unity => {
					version_config.push_str("engine:\n  unity: {}\n\n");
				}
				InitEngine::Unreal => {
					version_config.push_str("engine:\n  unreal: {}\n\n");
				}
				InitEngine::Godot => {
					version_config.push_str("engine:\n  godot: {}\n\n");
				}
				InitEngine::HTML5 => {
					version_config.push_str("engine:\n  html5: {}\n\n");
				}
				InitEngine::Custom => {
					// Do nothing
				}
			}

			// Write file
			fs::write(current_dir.join("rivet.yaml"), version_config)?;

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
