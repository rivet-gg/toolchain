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
pub struct MatchmakerLobbiesSetClosedRequest {
	#[serde(rename = "is_closed")]
	pub is_closed: bool,
}

impl MatchmakerLobbiesSetClosedRequest {
	pub fn new(is_closed: bool) -> MatchmakerLobbiesSetClosedRequest {
		MatchmakerLobbiesSetClosedRequest { is_closed }
	}
}