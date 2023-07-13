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
pub struct AuthCompleteEmailVerificationResponse {
    #[serde(rename = "status")]
    pub status: crate::models::AuthCompleteStatus,
}

impl AuthCompleteEmailVerificationResponse {
    pub fn new(status: crate::models::AuthCompleteStatus) -> AuthCompleteEmailVerificationResponse {
        AuthCompleteEmailVerificationResponse {
            status,
        }
    }
}


