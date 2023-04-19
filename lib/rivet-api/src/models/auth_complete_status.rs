/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// AuthCompleteStatus : Represents the state of an external account linking process.

/// Represents the state of an external account linking process.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum AuthCompleteStatus {
	#[serde(rename = "switch_identity")]
	SwitchIdentity,
	#[serde(rename = "linked_account_added")]
	LinkedAccountAdded,
	#[serde(rename = "already_complete")]
	AlreadyComplete,
	#[serde(rename = "expired")]
	Expired,
	#[serde(rename = "too_many_attempts")]
	TooManyAttempts,
	#[serde(rename = "incorrect")]
	Incorrect,
}

impl ToString for AuthCompleteStatus {
	fn to_string(&self) -> String {
		match self {
			Self::SwitchIdentity => String::from("switch_identity"),
			Self::LinkedAccountAdded => String::from("linked_account_added"),
			Self::AlreadyComplete => String::from("already_complete"),
			Self::Expired => String::from("expired"),
			Self::TooManyAttempts => String::from("too_many_attempts"),
			Self::Incorrect => String::from("incorrect"),
		}
	}
}

impl Default for AuthCompleteStatus {
	fn default() -> AuthCompleteStatus {
		Self::SwitchIdentity
	}
}
