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
pub struct PartyMemberState {
    #[serde(rename = "idle", skip_serializing_if = "Option::is_none")]
    pub idle: Option<serde_json::Value>,
    #[serde(rename = "matchmaker_finding_lobby", skip_serializing_if = "Option::is_none")]
    pub matchmaker_finding_lobby: Option<serde_json::Value>,
    #[serde(rename = "matchmaker_lobby", skip_serializing_if = "Option::is_none")]
    pub matchmaker_lobby: Option<Box<crate::models::PartyMemberStateMatchmakerLobby>>,
    #[serde(rename = "matchmaker_pending", skip_serializing_if = "Option::is_none")]
    pub matchmaker_pending: Option<serde_json::Value>,
}

impl PartyMemberState {
    pub fn new() -> PartyMemberState {
        PartyMemberState {
            idle: None,
            matchmaker_finding_lobby: None,
            matchmaker_lobby: None,
            matchmaker_pending: None,
        }
    }
}


