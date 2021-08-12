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
pub struct LobbyGroupRuntimeDocker {
    #[serde(rename = "build_id", skip_serializing_if = "Option::is_none")]
    pub build_id: Option<String>,
    #[serde(rename = "args")]
    pub args: Vec<String>,
    #[serde(rename = "ports")]
    pub ports: Vec<crate::models::LobbyGroupRuntimeDockerPorts>,
    #[serde(rename = "env_vars")]
    pub env_vars: Vec<crate::models::LobbyGroupRuntimeDockerEnvVars>,
}

impl LobbyGroupRuntimeDocker {
    pub fn new(args: Vec<String>, ports: Vec<crate::models::LobbyGroupRuntimeDockerPorts>, env_vars: Vec<crate::models::LobbyGroupRuntimeDockerEnvVars>) -> LobbyGroupRuntimeDocker {
        LobbyGroupRuntimeDocker {
            build_id: None,
            args,
            ports,
            env_vars,
        }
    }
}


