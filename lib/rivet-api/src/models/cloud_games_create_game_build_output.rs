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
pub struct CloudGamesCreateGameBuildOutput {
    /// A universally unique identifier.
    #[serde(rename = "build_id")]
    pub build_id: String,
    #[serde(rename = "image_presigned_request")]
    pub image_presigned_request: Box<crate::models::UploadPresignedRequest>,
    /// A universally unique identifier.
    #[serde(rename = "upload_id")]
    pub upload_id: String,
}

impl CloudGamesCreateGameBuildOutput {
    pub fn new(build_id: String, image_presigned_request: crate::models::UploadPresignedRequest, upload_id: String) -> CloudGamesCreateGameBuildOutput {
        CloudGamesCreateGameBuildOutput {
            build_id,
            image_presigned_request: Box::new(image_presigned_request),
            upload_id,
        }
    }
}


