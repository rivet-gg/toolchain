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
pub struct IdentityListFriendsResponse {
    #[serde(rename = "anchor", skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    #[serde(rename = "identities")]
    pub identities: Vec<crate::models::IdentityHandle>,
    #[serde(rename = "watch")]
    pub watch: Box<crate::models::WatchResponse>,
}

impl IdentityListFriendsResponse {
    pub fn new(identities: Vec<crate::models::IdentityHandle>, watch: crate::models::WatchResponse) -> IdentityListFriendsResponse {
        IdentityListFriendsResponse {
            anchor: None,
            identities,
            watch: Box::new(watch),
        }
    }
}


