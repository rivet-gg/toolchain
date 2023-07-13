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
#[serde(deny_unknown_fields)]
pub struct GroupCreateResponse {
    #[serde(rename = "group_id")]
    pub group_id: uuid::Uuid,
}

impl GroupCreateResponse {
    pub fn new(group_id: uuid::Uuid) -> GroupCreateResponse {
        GroupCreateResponse {
            group_id,
        }
    }
}


