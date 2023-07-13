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
pub struct IdentityGetHandlesResponse {
    #[serde(rename = "identities")]
    pub identities: Vec<crate::models::IdentityHandle>,
    #[serde(rename = "watch")]
    pub watch: Box<crate::models::WatchResponse>,
}

impl IdentityGetHandlesResponse {
    pub fn new(identities: Vec<crate::models::IdentityHandle>, watch: crate::models::WatchResponse) -> IdentityGetHandlesResponse {
        IdentityGetHandlesResponse {
            identities,
            watch: Box::new(watch),
        }
    }
}


