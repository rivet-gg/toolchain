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
#[serde(deny_unknown_fields)]
pub struct PartySetPublicityRequest {
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<crate::models::PartyPublicityLevel>,
    #[serde(rename = "mutual_followers", skip_serializing_if = "Option::is_none")]
    pub mutual_followers: Option<crate::models::PartyPublicityLevel>,
    #[serde(rename = "public", skip_serializing_if = "Option::is_none")]
    pub public: Option<crate::models::PartyPublicityLevel>,
}

impl PartySetPublicityRequest {
    pub fn new() -> PartySetPublicityRequest {
        PartySetPublicityRequest {
            groups: None,
            mutual_followers: None,
            public: None,
        }
    }
}


