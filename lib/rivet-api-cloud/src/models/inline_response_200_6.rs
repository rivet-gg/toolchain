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
pub struct InlineResponse2006 {
    #[serde(rename = "namespace")]
    pub namespace: Box<crate::models::NamespaceFull>,
}

impl InlineResponse2006 {
    pub fn new(namespace: crate::models::NamespaceFull) -> InlineResponse2006 {
        InlineResponse2006 {
            namespace: Box::new(namespace),
        }
    }
}
