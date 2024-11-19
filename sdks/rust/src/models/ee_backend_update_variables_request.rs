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
pub struct EeBackendUpdateVariablesRequest {
	#[serde(rename = "variables")]
	pub variables: ::std::collections::HashMap<String, crate::models::EeBackendUpdateVariable>,
}

impl EeBackendUpdateVariablesRequest {
	pub fn new(
		variables: ::std::collections::HashMap<String, crate::models::EeBackendUpdateVariable>,
	) -> EeBackendUpdateVariablesRequest {
		EeBackendUpdateVariablesRequest { variables }
	}
}
