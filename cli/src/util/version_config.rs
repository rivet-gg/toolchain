use std::{
	convert::{TryFrom, TryInto},
	str::FromStr,
};

use cli_core::rivet_api::models;
use global_error::prelude::*;
use serde_json::json;

use crate::util::text;

pub enum Engine {
	Unity,
	Unreal,
	Godot,
	Html5,
	Custom,
}

impl FromStr for Engine {
	type Err = GlobalError;

	fn from_str(s: &str) -> GlobalResult<Self> {
		match s.to_lowercase().as_str() {
			"unity" => Ok(Engine::Unity),
			"unreal" => Ok(Engine::Unreal),
			"godot" => Ok(Engine::Godot),
			"html5" => Ok(Engine::Html5),
			"custom" => Ok(Engine::Custom),
			_ => bail!("Invalid engine"),
		}
	}
}

impl Engine {
	pub fn learn_url(&self) -> String {
		match self {
			Engine::Unity => "https://rivet.gg/learn/unity".to_string(),
			Engine::Unreal { .. } => "https://rivet.gg/learn/unreal".to_string(),
			Engine::Godot => "https://rivet.gg/learn/godot".to_string(),
			Engine::Html5 => "https://rivet.gg/learn/html5".to_string(),
			Engine::Custom => "https://rivet.gg/learn/custom".to_string(),
		}
	}
}

impl TryInto<models::CloudVersionEngineConfig> for &Engine {
	type Error = GlobalError;
	fn try_into(self) -> Result<models::CloudVersionEngineConfig, Self::Error> {
		match self {
			Engine::Unity => Ok(models::CloudVersionEngineConfig {
				unity: Some(json!({})),
				..Default::default()
			}),
			Engine::Unreal => {
				bail!("cannot write engine for unreal")
			}
			Engine::Godot => Ok(models::CloudVersionEngineConfig {
				godot: Some(json!({})),
				..Default::default()
			}),
			Engine::Html5 => Ok(models::CloudVersionEngineConfig {
				html5: Some(json!({})),
				..Default::default()
			}),
			Engine::Custom => Ok(models::CloudVersionEngineConfig {
				custom: Some(json!({})),
				..Default::default()
			}),
		}
	}
}

impl TryFrom<&models::CloudVersionEngineConfig> for Engine {
	type Error = GlobalError;
	fn try_from(engine: &models::CloudVersionEngineConfig) -> Result<Self, Self::Error> {
		if engine.unity.is_some() {
			Ok(Engine::Unity)
		} else if engine.unreal.is_some() {
			bail!("cannot write engine for unreal")
		} else if engine.godot.is_some() {
			Ok(Engine::Godot)
		} else if engine.html5.is_some() {
			Ok(Engine::Html5)
		} else if engine.custom.is_some() {
			Ok(Engine::Custom)
		} else {
			bail!("no engine specified")
		}
	}
}

/// Generate a rivet.yaml file
pub fn generate(engine: &Engine, write_engine: bool) -> GlobalResult<String> {
	let mut version_config = String::new();

	// Render JSON spec
	version_config
		.push_str("# yaml-language-server: $schema=https://rivet.gg/rivet.schema.json\n\n");

	// Render info box
	let info_width = 78; // Standard 80 width - 2 for "# "
	let box_str = text::render_box_padded(&format!("\nThis configuration file is empty.\n\nGet started: {learn_url}\nReference: https://rivet.gg/docs/general/config\n", learn_url = engine.learn_url()), 4);
	let box_str = text::center_text(&box_str, info_width);
	let box_str = box_str.trim_end();
	let commented_info = box_str
		.lines()
		.map(|x| format!("# {x}"))
		.collect::<Vec<_>>()
		.join("\n");
	version_config.push_str(&commented_info);
	version_config.push_str("\n\n");

	// Add engine config
	if write_engine {
		let partial_config = crate::commands::config::CloudVersionConfigPartial {
			engine: Some(Box::new(engine.try_into()?)),
		};
		let engine_yaml = serde_yaml::to_string(&partial_config)?;
		version_config.push_str(&engine_yaml);
	}

	version_config.push_str("\n");

	Ok(version_config)
}
