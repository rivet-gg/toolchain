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
pub struct ServersListBuildsResponse {
    /// A list of builds for the game associated with the token.
    #[serde(rename = "builds")]
    pub builds: Vec<crate::models::CloudBuildSummary>,
}

impl ServersListBuildsResponse {
    pub fn new(builds: Vec<crate::models::CloudBuildSummary>) -> ServersListBuildsResponse {
        ServersListBuildsResponse {
            builds,
        }
    }
}


