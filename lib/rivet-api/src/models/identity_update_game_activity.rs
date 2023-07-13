/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IdentityUpdateGameActivity : Information about the identity's current game. This is information that all other identities can see about what the current identity is doing.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IdentityUpdateGameActivity {
    /// A short message about the current game activity.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// JSON data seen only by the given identity and their mutual followers.
    #[serde(rename = "mutual_metadata", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub mutual_metadata: Option<Option<serde_json::Value>>,
    /// JSON data seen by anyone.
    #[serde(rename = "public_metadata", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub public_metadata: Option<Option<serde_json::Value>>,
}

impl IdentityUpdateGameActivity {
    /// Information about the identity's current game. This is information that all other identities can see about what the current identity is doing.
    pub fn new() -> IdentityUpdateGameActivity {
        IdentityUpdateGameActivity {
            message: None,
            mutual_metadata: None,
            public_metadata: None,
        }
    }
}


