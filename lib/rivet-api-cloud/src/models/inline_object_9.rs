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
pub struct InlineObject9 {
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "config")]
    pub config: Box<crate::models::CloudVersionConfig>,
}

impl InlineObject9 {
    pub fn new(display_name: String, config: crate::models::CloudVersionConfig) -> InlineObject9 {
        InlineObject9 {
            display_name,
            config: Box::new(config),
        }
    }
}


