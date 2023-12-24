/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// MatchmakerJoinLobby : A matchmaker lobby.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MatchmakerJoinLobby {
	#[serde(rename = "lobby_id")]
	pub lobby_id: uuid::Uuid,
	#[serde(rename = "player")]
	pub player: Box<crate::models::MatchmakerJoinPlayer>,
	/// **Deprecated**
	#[serde(rename = "ports")]
	pub ports: ::std::collections::HashMap<String, crate::models::MatchmakerJoinPort>,
	#[serde(rename = "region")]
	pub region: Box<crate::models::MatchmakerJoinRegion>,
}

impl MatchmakerJoinLobby {
	/// A matchmaker lobby.
	pub fn new(
		lobby_id: uuid::Uuid,
		player: crate::models::MatchmakerJoinPlayer,
		ports: ::std::collections::HashMap<String, crate::models::MatchmakerJoinPort>,
		region: crate::models::MatchmakerJoinRegion,
	) -> MatchmakerJoinLobby {
		MatchmakerJoinLobby {
			lobby_id,
			player: Box::new(player),
			ports,
			region: Box::new(region),
		}
	}
}