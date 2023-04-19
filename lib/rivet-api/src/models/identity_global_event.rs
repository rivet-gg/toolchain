/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// IdentityGlobalEvent : An event relevant to the current identity.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IdentityGlobalEvent {
	#[serde(rename = "kind")]
	pub kind: Box<crate::models::IdentityGlobalEventKind>,
	#[serde(rename = "notification", skip_serializing_if = "Option::is_none")]
	pub notification: Option<Box<crate::models::IdentityGlobalEventNotification>>,
	#[serde(rename = "ts")]
	pub ts: String,
}

impl IdentityGlobalEvent {
	/// An event relevant to the current identity.
	pub fn new(kind: crate::models::IdentityGlobalEventKind, ts: String) -> IdentityGlobalEvent {
		IdentityGlobalEvent {
			kind: Box::new(kind),
			notification: None,
			ts,
		}
	}
}
