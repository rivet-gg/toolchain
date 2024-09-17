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
pub struct EeBackendGetResponse {
    #[serde(rename = "backend")]
    pub backend: Box<crate::models::EeBackendBackend>,
    #[serde(rename = "watch")]
    pub watch: Box<crate::models::WatchResponse>,
}

impl EeBackendGetResponse {
    pub fn new(backend: crate::models::EeBackendBackend, watch: crate::models::WatchResponse) -> EeBackendGetResponse {
        EeBackendGetResponse {
            backend: Box::new(backend),
            watch: Box::new(watch),
        }
    }
}

