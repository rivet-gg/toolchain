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
pub struct ActorGameGuardRouting {
	#[serde(rename = "authorization", skip_serializing_if = "Option::is_none")]
	pub authorization: Option<Box<crate::models::ActorPortAuthorization>>,
}

impl ActorGameGuardRouting {
	pub fn new() -> ActorGameGuardRouting {
		ActorGameGuardRouting {
			authorization: None,
		}
	}
}
