/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MatchmakerGameModeStatistics {
	#[serde(rename = "player_count")]
	pub player_count: i64,
	#[serde(rename = "regions")]
	pub regions: ::std::collections::HashMap<String, crate::models::MatchmakerRegionStatistics>,
}

impl MatchmakerGameModeStatistics {
	pub fn new(
		player_count: i64,
		regions: ::std::collections::HashMap<String, crate::models::MatchmakerRegionStatistics>,
	) -> MatchmakerGameModeStatistics {
		MatchmakerGameModeStatistics {
			player_count,
			regions,
		}
	}
}
