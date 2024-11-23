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
pub struct CloudGamesNamespacesUpdateGameNamespaceVersionRequest {
	#[serde(rename = "version_id")]
	pub version_id: uuid::Uuid,
}

impl CloudGamesNamespacesUpdateGameNamespaceVersionRequest {
	pub fn new(version_id: uuid::Uuid) -> CloudGamesNamespacesUpdateGameNamespaceVersionRequest {
		CloudGamesNamespacesUpdateGameNamespaceVersionRequest { version_id }
	}
}
