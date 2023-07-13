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
pub struct IdentityListMutualFriendsResponse {
    #[serde(rename = "anchor", skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    #[serde(rename = "identities")]
    pub identities: Vec<crate::models::IdentityHandle>,
}

impl IdentityListMutualFriendsResponse {
    pub fn new(identities: Vec<crate::models::IdentityHandle>) -> IdentityListMutualFriendsResponse {
        IdentityListMutualFriendsResponse {
            anchor: None,
            identities,
        }
    }
}


