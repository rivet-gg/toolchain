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
pub struct CloudGamesNamespacesListNamespaceLobbiesResponse {
    /// A list of lobby log summaries.
    #[serde(rename = "lobbies")]
    pub lobbies: Vec<crate::models::CloudLogsLobbySummary>,
}

impl CloudGamesNamespacesListNamespaceLobbiesResponse {
    pub fn new(lobbies: Vec<crate::models::CloudLogsLobbySummary>) -> CloudGamesNamespacesListNamespaceLobbiesResponse {
        CloudGamesNamespacesListNamespaceLobbiesResponse {
            lobbies,
        }
    }
}


