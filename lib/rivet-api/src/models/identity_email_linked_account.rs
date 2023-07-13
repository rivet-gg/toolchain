/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IdentityEmailLinkedAccount : An identity's linked email.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IdentityEmailLinkedAccount {
    /// A valid email address
    #[serde(rename = "email")]
    pub email: String,
}

impl IdentityEmailLinkedAccount {
    /// An identity's linked email.
    pub fn new(email: String) -> IdentityEmailLinkedAccount {
        IdentityEmailLinkedAccount {
            email,
        }
    }
}


