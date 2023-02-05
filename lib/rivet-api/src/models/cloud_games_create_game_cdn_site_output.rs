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
pub struct CloudGamesCreateGameCdnSiteOutput {
    #[serde(rename = "presigned_requests")]
    pub presigned_requests: Vec<crate::models::UploadPresignedRequest>,
    /// A universally unique identifier.
    #[serde(rename = "site_id")]
    pub site_id: String,
    /// A universally unique identifier.
    #[serde(rename = "upload_id")]
    pub upload_id: String,
}

impl CloudGamesCreateGameCdnSiteOutput {
    pub fn new(presigned_requests: Vec<crate::models::UploadPresignedRequest>, site_id: String, upload_id: String) -> CloudGamesCreateGameCdnSiteOutput {
        CloudGamesCreateGameCdnSiteOutput {
            presigned_requests,
            site_id,
            upload_id,
        }
    }
}


