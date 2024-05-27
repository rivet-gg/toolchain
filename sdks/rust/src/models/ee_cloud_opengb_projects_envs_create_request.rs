/*
 * Rivet API EE
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EeCloudOpengbProjectsEnvsCreateRequest {
    /// Represent a resource's readable display name.
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// A human readable short identifier used to references resources. Different than a `uuid` because this is intended to be human readable. Different than `DisplayName` because this should not include special characters and be short.
    #[serde(rename = "name_id")]
    pub name_id: String,
    #[serde(rename = "tier")]
    pub tier: crate::models::EeOpengbTier,
}

impl EeCloudOpengbProjectsEnvsCreateRequest {
    pub fn new(display_name: String, name_id: String, tier: crate::models::EeOpengbTier) -> EeCloudOpengbProjectsEnvsCreateRequest {
        EeCloudOpengbProjectsEnvsCreateRequest {
            display_name,
            name_id,
            tier,
        }
    }
}

