/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CloudNamespaceVersion : A previously deployed namespace version.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CloudNamespaceVersion {
    /// RFC3339 timestamp.
    #[serde(rename = "deploy_ts")]
    pub deploy_ts: String,
    /// A universally unique identifier.
    #[serde(rename = "namespace_id")]
    pub namespace_id: String,
    /// A universally unique identifier.
    #[serde(rename = "version_id")]
    pub version_id: String,
}

impl CloudNamespaceVersion {
    /// A previously deployed namespace version.
    pub fn new(deploy_ts: String, namespace_id: String, version_id: String) -> CloudNamespaceVersion {
        CloudNamespaceVersion {
            deploy_ts,
            namespace_id,
            version_id,
        }
    }
}


