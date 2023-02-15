/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CloudVersionMatchmakerGameModeRuntimeDocker : A game mode runtime running through Docker.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudVersionMatchmakerGameModeRuntimeDocker {
    #[serde(rename = "args", skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    /// Client-side configuration
    #[serde(rename = "dockerfile", skip_serializing_if = "Option::is_none")]
    pub dockerfile: Option<String>,
    #[serde(rename = "env", skip_serializing_if = "Option::is_none")]
    pub env: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<uuid::Uuid>,
    #[serde(rename = "network_mode", skip_serializing_if = "Option::is_none")]
    pub network_mode: Option<crate::models::CloudVersionMatchmakerNetworkMode>,
    #[serde(rename = "ports", skip_serializing_if = "Option::is_none")]
    pub ports: Option<::std::collections::HashMap<String, crate::models::CloudVersionMatchmakerGameModeRuntimeDockerPort>>,
}

impl CloudVersionMatchmakerGameModeRuntimeDocker {
    /// A game mode runtime running through Docker.
    pub fn new() -> CloudVersionMatchmakerGameModeRuntimeDocker {
        CloudVersionMatchmakerGameModeRuntimeDocker {
            args: None,
            dockerfile: None,
            env: None,
            image: None,
            network_mode: None,
            ports: None,
        }
    }
}

