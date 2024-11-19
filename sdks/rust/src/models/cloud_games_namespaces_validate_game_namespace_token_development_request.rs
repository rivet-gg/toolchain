/*
 * Rivet API EE
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudGamesNamespacesValidateGameNamespaceTokenDevelopmentRequest {
	#[serde(rename = "hostname")]
	pub hostname: String,
	/// A list of docker ports.
	#[serde(rename = "lobby_ports")]
	pub lobby_ports: Vec<crate::models::CloudVersionMatchmakerLobbyGroupRuntimeDockerPort>,
}

impl CloudGamesNamespacesValidateGameNamespaceTokenDevelopmentRequest {
	pub fn new(
		hostname: String,
		lobby_ports: Vec<crate::models::CloudVersionMatchmakerLobbyGroupRuntimeDockerPort>,
	) -> CloudGamesNamespacesValidateGameNamespaceTokenDevelopmentRequest {
		CloudGamesNamespacesValidateGameNamespaceTokenDevelopmentRequest {
			hostname,
			lobby_ports,
		}
	}
}
