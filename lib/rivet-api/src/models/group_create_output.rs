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
pub struct GroupCreateOutput {
    /// A universally unique identifier.
    #[serde(rename = "group_id")]
    pub group_id: String,
}

impl GroupCreateOutput {
    pub fn new(group_id: String) -> GroupCreateOutput {
        GroupCreateOutput {
            group_id,
        }
    }
}


