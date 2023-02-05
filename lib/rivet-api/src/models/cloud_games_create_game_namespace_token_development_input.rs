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
pub struct CloudGamesCreateGameNamespaceTokenDevelopmentInput {
    /// The hostname used for the token.
    #[serde(rename = "hostname")]
    pub hostname: String,
    /// A list of docker ports.
    #[serde(rename = "lobby_ports")]
    pub lobby_ports: Vec<crate::models::CloudVersionMatchmakerLobbyGroupRuntimeDockerPort>,
}

impl CloudGamesCreateGameNamespaceTokenDevelopmentInput {
    pub fn new(hostname: String, lobby_ports: Vec<crate::models::CloudVersionMatchmakerLobbyGroupRuntimeDockerPort>) -> CloudGamesCreateGameNamespaceTokenDevelopmentInput {
        CloudGamesCreateGameNamespaceTokenDevelopmentInput {
            hostname,
            lobby_ports,
        }
    }
}


