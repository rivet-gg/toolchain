/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CloudGamesGameSummary : A game summary.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudGamesGameSummary {
    /// The URL of this game's banner image.
    #[serde(rename = "banner_url", skip_serializing_if = "Option::is_none")]
    pub banner_url: Option<String>,
    /// RFC3339 timestamp.
    #[serde(rename = "create_ts")]
    pub create_ts: String,
    /// A universally unique identifier.
    #[serde(rename = "developer_group_id")]
    pub developer_group_id: String,
    /// Represent a resource's readable display name.
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// A universally unique identifier.
    #[serde(rename = "game_id")]
    pub game_id: String,
    /// The URL of this game's logo image.
    #[serde(rename = "logo_url", skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    /// A human readable short identifier used to references resources. Different than a `rivet.common#Uuid` because this is intended to be human readable. Different than `rivet.common#DisplayName` because this should not include special characters and be short.
    #[serde(rename = "name_id")]
    pub name_id: String,
    /// Unsigned 32 bit integer.
    #[serde(rename = "total_player_count", skip_serializing_if = "Option::is_none")]
    pub total_player_count: Option<i32>,
}

impl CloudGamesGameSummary {
    /// A game summary.
    pub fn new(create_ts: String, developer_group_id: String, display_name: String, game_id: String, name_id: String) -> CloudGamesGameSummary {
        CloudGamesGameSummary {
            banner_url: None,
            create_ts,
            developer_group_id,
            display_name,
            game_id,
            logo_url: None,
            name_id,
            total_player_count: None,
        }
    }
}

