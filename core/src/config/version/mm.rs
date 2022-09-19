use serde::Deserialize;
use std::collections::HashMap;

use crate::error::Error;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Matchmaker {
	pub game_modes: HashMap<String, game_mode::GameMode>,
	#[serde(default)]
	pub captcha: Option<captcha::Captcha>,

	// Runtime overrides
	#[serde(default)]
	docker: Option<DockerOverride>,
}

// TODO: Remove clone
#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct DockerOverride {
	#[serde(default)]
	build: Option<String>,
}

pub mod game_mode {
	use serde::Deserialize;
	use std::collections::HashMap;

	#[derive(Debug, Deserialize)]
	#[serde(deny_unknown_fields)]
	pub struct GameMode {
		pub regions: HashMap<String, Region>,
		pub max_players: MaxPlayers,
		#[serde(flatten)]
		pub runtime: runtime::Runtime,

		// Region overrides
		#[serde(default = "Region::default_tier")]
		pub tier: String,
		#[serde(default)]
		pub idle_lobbies: IdleLobbies,
	}

	#[derive(Debug, Deserialize)]
	#[serde(deny_unknown_fields)]
	pub struct Region {
		#[serde(default = "Region::default_tier")]
		pub tier: String,
		#[serde(default)]
		pub idle_lobbies: IdleLobbies,
	}

	#[derive(Debug, Deserialize)]
	#[serde(deny_unknown_fields)]
	pub struct IdleLobbies {
		pub min: u32,
		pub max: u32,
	}

	#[derive(Debug, Deserialize)]
	#[serde(deny_unknown_fields)]
	#[serde(untagged)]
	pub enum MaxPlayers {
		Universal(u32),
		Split(MaxPlayersSplit),
	}

	#[derive(Debug, Deserialize)]
	#[serde(deny_unknown_fields)]
	pub struct MaxPlayersSplit {
		pub normal: u32,
		pub direct: u32,
		pub party: u32,
	}

	pub mod runtime {
		use serde::Deserialize;

		use crate::{config::version::mm::DockerOverride, error::Error};

		#[derive(Debug, Deserialize)]
		#[serde(rename_all = "snake_case", deny_unknown_fields)]
		pub enum Runtime {
			Docker(docker::Docker),
		}

		pub mod docker {
			use std::collections::HashMap;

			use serde::Deserialize;

			#[derive(Debug, Deserialize)]
			#[serde(deny_unknown_fields)]
			pub struct Docker {
				pub build: Option<String>,
				pub ports: HashMap<String, Port>,
				#[serde(default)]
				pub args: Vec<String>,
				#[serde(default)]
				pub env: HashMap<String, String>,
			}

			#[derive(Debug, Deserialize)]
			#[serde(deny_unknown_fields)]
			pub struct Port {
				pub target: u32,
				pub proto: ProxyProtocol,
			}

			#[derive(Debug, Deserialize)]
			#[serde(rename_all = "kebab-case", deny_unknown_fields)]
			pub enum ProxyProtocol {
				Http,
				Https,
			}

			impl ProxyProtocol {
				pub fn build_model(&self) -> rivet_cloud::model::ProxyProtocol {
					match self {
						ProxyProtocol::Http => rivet_cloud::model::ProxyProtocol::Http,
						ProxyProtocol::Https => rivet_cloud::model::ProxyProtocol::Https,
					}
				}
			}
		}

		impl Runtime {
			pub fn build_model(
				&self,
				_game: &rivet_cloud::model::GameFull,
				docker_override: &Option<DockerOverride>,
			) -> Result<rivet_cloud::model::LobbyGroupRuntime, Error> {
				use rivet_cloud::model::*;

				let runtime = match self {
					Runtime::Docker(docker) => LobbyGroupRuntime::Docker(
						LobbyGroupRuntimeDocker::builder()
							.build_id(
								docker
									.build
									.clone()
									.or_else(|| {
										docker_override.as_ref().and_then(|x| x.build.clone())
									})
									.ok_or_else(|| {
										Error::config(
											"matchmaker.game_mode.*.docker.build",
											"missing build",
										)
									})?,
							)
							.set_args(Some(docker.args.clone()))
							.set_ports(Some(
								docker
									.ports
									.iter()
									.map(|(label, port)| {
										LobbyGroupRuntimeDockerPort::builder()
											.label(label)
											.target_port(port.target as i32)
											.proxy_protocol(port.proto.build_model())
											.build()
									})
									.collect(),
							))
							.set_env_vars(Some(
								docker
									.env
									.iter()
									.map(|(key, value)| {
										LobbyGroupRuntimeDockerEnvVar::builder()
											.key(key)
											.value(value)
											.build()
									})
									.collect(),
							))
							.build(),
					),
				};

				Ok(runtime)
			}
		}
	}

	impl Region {
		fn default_tier() -> String {
			"basic-1d1".into()
		}
	}

	impl Default for IdleLobbies {
		fn default() -> Self {
			Self { min: 0, max: 1 }
		}
	}

	impl MaxPlayers {
		pub fn normal(&self) -> u32 {
			match *self {
				MaxPlayers::Universal(x) => x,
				MaxPlayers::Split(MaxPlayersSplit { normal, .. }) => normal,
			}
		}

		pub fn direct(&self) -> u32 {
			match *self {
				MaxPlayers::Universal(x) => x,
				MaxPlayers::Split(MaxPlayersSplit { direct, .. }) => direct,
			}
		}

		pub fn party(&self) -> u32 {
			match *self {
				MaxPlayers::Universal(x) => x,
				MaxPlayers::Split(MaxPlayersSplit { party, .. }) => party,
			}
		}
	}
}

pub mod captcha {
	use serde::Deserialize;

	#[derive(Debug, Deserialize)]
	#[serde(deny_unknown_fields)]
	pub struct Captcha {
		pub hcaptcha: Option<Hcaptcha>,
		pub requests_before_reverify: u32,
		pub verification_ttl: u64,
	}

	#[derive(Debug, Deserialize)]
	#[serde(deny_unknown_fields)]
	pub struct Hcaptcha {
		pub level: Level,
	}

	#[derive(Debug, Deserialize)]
	#[serde(rename_all = "kebab-case", deny_unknown_fields)]
	pub enum Level {
		Easy,
		Moderate,
		Difficult,
		AlwaysOn,
	}
}

// TODO: Don't consume self
impl Matchmaker {
	pub fn build_model(
		&self,
		game: &rivet_cloud::model::GameFull,
	) -> Result<rivet_cloud::model::MatchmakerVersionConfig, Error> {
		use rivet_cloud::model::*;

		let available_regions = game
			.available_regions()
			.ok_or_else(|| Error::internal("game.available_regions"))?;

		let lobby_groups =
			self.game_modes
				.iter()
				.map(|(game_mode_name_id, game_mode)| {
					// TODO: Add region-specific config

					let regions =
						available_regions
							.iter()
							.map(|region_summary| {
								Ok(LobbyGroupRegion::builder()
									.region_id(region_summary.region_id().ok_or_else(|| {
										Error::internal("region_summary.region_id")
									})?)
									.tier_name_id(&game_mode.tier)
									.idle_lobbies(
										IdleLobbiesConfig::builder()
											.min_idle_lobbies(game_mode.idle_lobbies.min as i32)
											.max_idle_lobbies(game_mode.idle_lobbies.max as i32)
											.build(),
									)
									.build())
							})
							.collect::<Result<Vec<_>, Error>>()?;

					let runtime = game_mode.runtime.build_model(game, &self.docker)?;

					Ok(LobbyGroup::builder()
						.name_id(game_mode_name_id)
						.set_regions(Some(regions))
						.max_players_normal(game_mode.max_players.normal() as i32)
						.max_players_direct(game_mode.max_players.direct() as i32)
						.max_players_party(game_mode.max_players.party() as i32)
						.runtime(runtime)
						.build())
				})
				.collect::<Result<Vec<_>, Error>>()?;

		let captcha = self.captcha.as_ref().map(|captcha| {
			MatchmakerCaptcha::builder()
				.set_hcaptcha(captcha.hcaptcha.as_ref().map(|hcaptcha| {
					MatchmakerCaptchaHcaptcha::builder()
						.level(match hcaptcha.level {
							captcha::Level::Easy => CaptchaLevel::Easy,
							captcha::Level::Moderate => CaptchaLevel::Moderate,
							captcha::Level::Difficult => CaptchaLevel::Difficult,
							captcha::Level::AlwaysOn => CaptchaLevel::AlwaysOn,
						})
						.build()
				}))
				.requests_before_reverify(captcha.requests_before_reverify as i32)
				.verification_ttl(captcha.verification_ttl as i64)
				.build()
		});

		Ok(MatchmakerVersionConfig::builder()
			.set_lobby_groups(Some(lobby_groups))
			.set_captcha(captcha)
			.build())
	}
}
