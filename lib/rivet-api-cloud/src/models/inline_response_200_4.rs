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
pub struct InlineResponse2004 {
    #[serde(rename = "version")]
    pub version: Box<crate::models::VersionFull>,
}

impl InlineResponse2004 {
    pub fn new(version: crate::models::VersionFull) -> InlineResponse2004 {
        InlineResponse2004 {
            version: Box::new(version),
        }
    }
}
