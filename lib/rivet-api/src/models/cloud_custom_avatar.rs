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
pub struct CloudCustomAvatar {
    /// A universally unique identifier.
    #[serde(rename = "upload_id")]
    pub upload_id: String,
}

impl CloudCustomAvatar {
    pub fn new(upload_id: String) -> CloudCustomAvatar {
        CloudCustomAvatar {
            upload_id,
        }
    }
}


