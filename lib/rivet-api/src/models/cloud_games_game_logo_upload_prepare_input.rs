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
pub struct CloudGamesGameLogoUploadPrepareInput {
    /// Unsigned 64 bit integer.
    #[serde(rename = "content_length", skip_serializing_if = "Option::is_none")]
    pub content_length: Option<f64>,
    /// The MIME type of the game logo.
    #[serde(rename = "mime", skip_serializing_if = "Option::is_none")]
    pub mime: Option<String>,
    /// The path/filename of the game logo.
    #[serde(rename = "path")]
    pub path: String,
}

impl CloudGamesGameLogoUploadPrepareInput {
    pub fn new(path: String) -> CloudGamesGameLogoUploadPrepareInput {
        CloudGamesGameLogoUploadPrepareInput {
            content_length: None,
            mime: None,
            path,
        }
    }
}


