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
pub struct EeBackendGetDbUrlResponse {
	#[serde(rename = "url", skip_serializing_if = "Option::is_none")]
	pub url: Option<String>,
}

impl EeBackendGetDbUrlResponse {
	pub fn new() -> EeBackendGetDbUrlResponse {
		EeBackendGetDbUrlResponse { url: None }
	}
}
