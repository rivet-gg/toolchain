/*
 * Rivet Cloud
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LobbyGroupRuntimeDockerEnvVars {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value")]
    pub value: String,
}

impl LobbyGroupRuntimeDockerEnvVars {
    pub fn new(key: String, value: String) -> LobbyGroupRuntimeDockerEnvVars {
        LobbyGroupRuntimeDockerEnvVars {
            key,
            value,
        }
    }
}


