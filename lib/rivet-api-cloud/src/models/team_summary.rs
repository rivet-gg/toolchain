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
pub struct TeamSummary {
    #[serde(rename = "team_id")]
    pub team_id: String,
    #[serde(rename = "create_ts")]
    pub create_ts: i32,
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "is_developer")]
    pub is_developer: bool,
}

impl TeamSummary {
    pub fn new(team_id: String, create_ts: i32, display_name: String, is_developer: bool) -> TeamSummary {
        TeamSummary {
            team_id,
            create_ts,
            display_name,
            is_developer,
        }
    }
}


