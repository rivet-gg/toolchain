use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Matchmaker {
	pub game_modes: HashMap<String, game_mode::GameMode>,
	#[serde(default)]
	pub captcha: Option<captcha::Captcha>,
}

pub mod game_mode {
	use serde::Deserialize;
	use std::collections::HashMap;

	#[derive(Debug, Deserialize)]
	pub struct GameMode {
		pub regions: HashMap<String, Region>,
		pub max_players: MaxPlayers,
		pub runtime: runtime::Runtime,
	}

	#[derive(Debug, Deserialize)]
	pub struct Region {
		#[serde(default = "Region::default_tier")]
		pub tier: String,
		#[serde(default)]
		pub idle_lobbies: IdleLobbies,
	}

	#[derive(Debug, Deserialize)]
	pub struct IdleLobbies {
		pub min: u32,
		pub max: u32,
	}

	#[derive(Debug, Deserialize)]
	#[serde(untagged)]
	pub enum MaxPlayers {
		Universal(u32),
		Split(MaxPlayersSplit),
	}

	#[derive(Debug, Deserialize)]
	pub struct MaxPlayersSplit {
		pub normal: u32,
		pub direct: u32,
		pub party: u32,
	}

	pub mod runtime {
		use serde::Deserialize;

		#[derive(Debug, Deserialize)]
		pub enum Runtime {
			Docker(docker::Docker),
		}

		pub mod docker {
			use std::collections::HashMap;

			use serde::Deserialize;

			#[derive(Debug, Deserialize)]
			pub struct Docker {
				pub build: String,
				pub ports: HashMap<String, Port>,
				#[serde(default)]
				pub args: Vec<String>,
				#[serde(default)]
				pub env: HashMap<String, String>,
			}

			#[derive(Debug, Deserialize)]
			pub struct Port {
				pub target: u32,
				pub proto: ProxyProtocol,
			}

			#[derive(Debug, Deserialize)]
			pub enum ProxyProtocol {
				Http,
				Https,
			}
		}
	}

	impl Region {
		fn default_tier() -> String {
			"basic-1d1".into()
		}
	}

	impl Default for Region {
		fn default() -> Self {
			Self {
				tier: Self::default_tier(),
				idle_lobbies: Default::default(),
			}
		}
	}

	impl Default for IdleLobbies {
		fn default() -> Self {
			Self { min: 0, max: 1 }
		}
	}
}

pub mod captcha {
	use serde::Deserialize;

	#[derive(Debug, Deserialize)]
	pub struct Captcha {
		pub hcaptcha: Option<Hcaptcha>,
		pub requests_before_reverify: u32,
		pub verification_ttl: u64,
	}

	#[derive(Debug, Deserialize)]
	pub struct Hcaptcha {
		pub level: Level,
	}

	#[derive(Debug, Deserialize)]
	pub enum Level {
		Easy,
		Moderate,
		Difficult,
		AlwaysOn,
	}
}
