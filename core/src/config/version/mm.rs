use serde::Deserialize;
use std::collections::HashMap;

use crate::error::Error;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Matchmaker {
	pub game_modes: HashMap<String, game_mode::GameMode>,
	#[serde(default)]
	pub captcha: Option<captcha::Captcha>,

	// Game mode overrides
	pub max_players: Option<game_mode::MaxPlayers>,

	// Region overrides
	#[serde(default, flatten)]
	pub region: game_mode::Region,

	// Runtime overrides
	#[serde(default)]
	pub docker: game_mode::runtime::docker::Docker,
}

pub mod game_mode {
	use serde::Deserialize;
	use std::collections::HashMap;

	#[derive(Debug, Deserialize)]
	#[serde(deny_unknown_fields)]
	pub struct GameMode {
		pub max_players: Option<MaxPlayers>,

		#[serde(default)]
		pub regions: Option<HashMap<String, Region>>,

		#[serde(flatten)]
		pub runtime: runtime::Runtime,

		// Region overrides
		#[serde(default, flatten)]
		pub region: Region,
	}

	#[derive(Debug, Default, Deserialize)]
	#[serde(deny_unknown_fields)]
	pub struct Region {
		#[serde(default)]
		pub tier: Option<String>,
		#[serde(default)]
		pub idle_lobbies: Option<IdleLobbies>,
	}

	#[derive(Clone, Debug, Deserialize)]
	#[serde(deny_unknown_fields)]
	pub struct IdleLobbies {
		pub min: u32,
		pub max: u32,
	}

	#[derive(Clone, Debug, Deserialize)]
	#[serde(untagged)]
	pub enum MaxPlayers {
		Universal(u32),
		Split(MaxPlayersSplit),
	}

	impl Default for MaxPlayers {
		fn default() -> Self {
			MaxPlayers::Split(MaxPlayersSplit {
				normal: 32,
				direct: 40,
				party: 40,
			})
		}
	}

	#[derive(Clone, Debug, Deserialize)]
	#[serde(deny_unknown_fields)]
	pub struct MaxPlayersSplit {
		pub normal: u32,
		pub direct: u32,
		pub party: u32,
	}

	pub mod runtime {
		use serde::Deserialize;

		use crate::error::Error;

		#[derive(Debug, Deserialize)]
		#[serde(rename_all = "snake_case")]
		pub enum Runtime {
			Docker(docker::Docker),
		}

		pub mod docker {
			use std::collections::HashMap;

			use serde::Deserialize;

			#[derive(Debug, Default, Deserialize)]
			#[serde(deny_unknown_fields)]
			pub struct Docker {
				pub build: Option<String>,
				#[serde(default)]
				pub args: Option<Vec<String>>,
				#[serde(default)]
				pub env: Option<HashMap<String, String>>,
				#[serde(default)]
				pub ports: Option<HashMap<String, Port>>,
				#[serde(default)]
				pub network_mode: Option<NetworkMode>,
			}

			#[derive(Clone, Debug, Deserialize)]
			#[serde(deny_unknown_fields)]
			pub struct Port {
				pub target: Option<u32>,
				pub range: Option<PortRange>,
				pub proto: ProxyProtocol,
			}

			#[derive(Clone, Debug, Deserialize)]
			#[serde(rename_all = "kebab-case", deny_unknown_fields)]
			pub struct PortRange {
				pub min: u16,
				pub max: u16,
			}

			#[derive(Clone, Debug, Deserialize)]
			#[serde(rename_all = "kebab-case")]
			pub enum ProxyProtocol {
				Http,
				Https,
				Udp,
			}

			impl ProxyProtocol {
				pub fn build_model(&self) -> rivet_cloud::model::ProxyProtocol {
					match self {
						ProxyProtocol::Http => rivet_cloud::model::ProxyProtocol::Http,
						ProxyProtocol::Https => rivet_cloud::model::ProxyProtocol::Https,
						ProxyProtocol::Udp => rivet_cloud::model::ProxyProtocol::Udp,
					}
				}
			}

			#[derive(Clone, Debug, Deserialize)]
			#[serde(rename_all = "kebab-case")]
			pub enum NetworkMode {
				Bridge,
				Host,
			}

			impl NetworkMode {
				pub fn build_model(&self) -> rivet_cloud::model::NetworkMode {
					match self {
						NetworkMode::Bridge => rivet_cloud::model::NetworkMode::Bridge,
						NetworkMode::Host => rivet_cloud::model::NetworkMode::Host,
					}
				}
			}

			impl Default for NetworkMode {
				fn default() -> Self {
					Self::Bridge
				}
			}
		}

		impl Runtime {
			pub fn build_model(
				&self,
				_game: &rivet_cloud::model::GameFull,
				docker_override: &docker::Docker,
			) -> Result<rivet_cloud::model::LobbyGroupRuntime, Error> {
				use rivet_cloud::model::*;

				let runtime = match self {
					Runtime::Docker(docker) => LobbyGroupRuntime::Docker(
						LobbyGroupRuntimeDocker::builder()
							.build_id(
								docker
									.build
									.clone()
									.or_else(|| docker_override.build.clone())
									.ok_or_else(|| {
										Error::config(
											"matchmaker.game_mode.*.docker.build",
											"missing build",
										)
									})?,
							)
							.set_args(Some(
								docker
									.args
									.clone()
									.or_else(|| docker_override.args.clone())
									.unwrap_or_default(),
							))
							.set_env_vars(Some(
								docker
									.env
									.clone()
									.or_else(|| docker_override.env.clone())
									.unwrap_or_default()
									.iter()
									.map(|(key, value)| {
										LobbyGroupRuntimeDockerEnvVar::builder()
											.key(key)
											.value(value)
											.build()
									})
									.collect(),
							))
							.network_mode(
								docker
									.network_mode
									.clone()
									.or_else(|| docker_override.network_mode.clone())
									.unwrap_or_default()
									.build_model(),
							)
							.set_ports(Some(
								docker
									.ports
									.clone()
									.ok_or_else(|| docker_override.ports.clone())
									.unwrap_or_default()
									.iter()
									.map(|(label, port)| {
										LobbyGroupRuntimeDockerPort::builder()
											.label(label)
											.set_target_port(port.target.map(|x| x as i32))
											.set_port_range(port.range.as_ref().map(|range| {
												PortRange::builder()
													.min(range.min as i32)
													.max(range.max as i32)
													.build()
											}))
											.proxy_protocol(port.proto.build_model())
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
		pub fn default_tier() -> String {
			"basic-1d1".into()
		}
	}

	impl Default for IdleLobbies {
		fn default() -> Self {
			Self { min: 1, max: 1 }
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
	#[serde(rename_all = "kebab-case")]
	pub enum Level {
		Easy,
		Moderate,
		Difficult,
		AlwaysOn,
	}
}

impl Matchmaker {
	pub fn build_model(
		&self,
		game: &rivet_cloud::model::GameFull,
	) -> Result<rivet_cloud::model::MatchmakerVersionConfig, Error> {
		use rivet_cloud::model::*;

		let available_regions = game
			.available_regions()
			.ok_or_else(|| Error::internal("game.available_regions"))?;

		let lobby_groups = self
			.game_modes
			.iter()
			.map(|(game_mode_name_id, game_mode)| {
				let max_players = game_mode
					.max_players
					.clone()
					.or_else(|| game_mode.max_players.clone())
					.unwrap_or_default();

				// Map provided regions to region summary
				let game_mode_regions = if let Some(regions) = &game_mode.regions {
					// Regions are provided
					regions
						.iter()
						.map(|(k, v)| {
							if let Some(summary) = available_regions
								.iter()
								.find(|x| x.region_name_id().map_or(false, |x| x == k))
							{
								Ok((summary, Some(v)))
							} else {
								Err(Error::RegionDoesNotExist {
									region_id: k.clone(),
								})
							}
						})
						.collect::<Result<Vec<_>, Error>>()?
				} else {
					// Use default regions
					available_regions
						.iter()
						.map(|x| (x, None))
						.collect::<Vec<_>>()
				};

				let regions = game_mode_regions
					.iter()
					.map(|(region_summary, region_config)| {
						let region_id = region_summary
							.region_id()
							.ok_or_else(|| Error::internal("region_summary.region_id"))
							.unwrap_or_default();

						// Derive region -> game mode config fallbacks
						let tier_name_id = region_config
							.and_then(|x| x.tier.clone())
							.or_else(|| game_mode.region.tier.clone())
							.or_else(|| self.region.tier.clone())
							.unwrap_or_else(game_mode::Region::default_tier);
						let idle_lobbies = region_config
							.and_then(|x| x.idle_lobbies.clone())
							.or_else(|| game_mode.region.idle_lobbies.clone())
							.or_else(|| self.region.idle_lobbies.clone())
							.unwrap_or_default();

						Ok(LobbyGroupRegion::builder()
							.region_id(region_id)
							.tier_name_id(tier_name_id)
							.idle_lobbies(
								IdleLobbiesConfig::builder()
									.min_idle_lobbies(idle_lobbies.min as i32)
									.max_idle_lobbies(idle_lobbies.max as i32)
									.build(),
							)
							.build())
					})
					.collect::<Result<Vec<_>, Error>>()?;

				let runtime = game_mode.runtime.build_model(game, &self.docker)?;

				Ok(LobbyGroup::builder()
					.name_id(game_mode_name_id)
					.set_regions(Some(regions))
					.max_players_normal(max_players.normal() as i32)
					.max_players_direct(max_players.direct() as i32)
					.max_players_party(max_players.party() as i32)
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
