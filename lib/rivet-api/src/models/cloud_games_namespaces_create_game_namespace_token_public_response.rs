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
pub struct CloudGamesNamespacesCreateGameNamespaceTokenPublicResponse {
    /// A JSON Web Token. Slightly modified to include a description prefix and use Protobufs of JSON.
    #[serde(rename = "token")]
    pub token: String,
}

impl CloudGamesNamespacesCreateGameNamespaceTokenPublicResponse {
    pub fn new(token: String) -> CloudGamesNamespacesCreateGameNamespaceTokenPublicResponse {
        CloudGamesNamespacesCreateGameNamespaceTokenPublicResponse {
            token,
        }
    }
}


