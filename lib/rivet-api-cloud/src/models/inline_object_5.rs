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
pub struct InlineObject5 {
    #[serde(rename = "files")]
    pub files: Vec<crate::models::UploadPrepareFile>,
    #[serde(rename = "display_name")]
    pub display_name: String,
}

impl InlineObject5 {
    pub fn new(
        files: Vec<crate::models::UploadPrepareFile>,
        display_name: String,
    ) -> InlineObject5 {
        InlineObject5 {
            files,
            display_name,
        }
    }
}
