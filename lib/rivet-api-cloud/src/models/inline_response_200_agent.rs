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
pub struct InlineResponse200Agent {
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::InlineResponse200AgentUser>>,
    #[serde(rename = "game_cloud", skip_serializing_if = "Option::is_none")]
    pub game_cloud: Option<Box<crate::models::InlineResponse200AgentGameCloud>>,
}

impl InlineResponse200Agent {
    pub fn new() -> InlineResponse200Agent {
        InlineResponse200Agent {
            user: None,
            game_cloud: None,
        }
    }
}
