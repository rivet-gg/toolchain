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
pub struct InlineResponse200AgentGameCloud {
    #[serde(rename = "game_id")]
    pub game_id: String,
}

impl InlineResponse200AgentGameCloud {
    pub fn new(game_id: String) -> InlineResponse200AgentGameCloud {
        InlineResponse200AgentGameCloud {
            game_id,
        }
    }
}


