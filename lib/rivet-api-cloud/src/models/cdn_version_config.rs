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
pub struct CdnVersionConfig {
    #[serde(rename = "site_id")]
    pub site_id: String,
}

impl CdnVersionConfig {
    pub fn new(site_id: String) -> CdnVersionConfig {
        CdnVersionConfig { site_id }
    }
}
