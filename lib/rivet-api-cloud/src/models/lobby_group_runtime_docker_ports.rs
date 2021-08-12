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
pub struct LobbyGroupRuntimeDockerPorts {
    #[serde(rename = "port")]
    pub port: i32,
    #[serde(rename = "enable_insecure_80")]
    pub enable_insecure_80: bool,
    #[serde(rename = "enable_secure_443")]
    pub enable_secure_443: bool,
}

impl LobbyGroupRuntimeDockerPorts {
    pub fn new(port: i32, enable_insecure_80: bool, enable_secure_443: bool) -> LobbyGroupRuntimeDockerPorts {
        LobbyGroupRuntimeDockerPorts {
            port,
            enable_insecure_80,
            enable_secure_443,
        }
    }
}

