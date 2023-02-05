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
pub struct CloudVersionMiddlewareKind {
    #[serde(rename = "custom_headers", skip_serializing_if = "Option::is_none")]
    pub custom_headers: Option<Box<crate::models::CloudVersionCustomHeadersMiddleware>>,
}

impl CloudVersionMiddlewareKind {
    pub fn new() -> CloudVersionMiddlewareKind {
        CloudVersionMiddlewareKind {
            custom_headers: None,
        }
    }
}


