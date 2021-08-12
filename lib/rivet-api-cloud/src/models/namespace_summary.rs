/*
 * Rivet Cloud
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NamespaceSummary {
    #[serde(rename = "namespace_id")]
    pub namespace_id: String,
    #[serde(rename = "create_ts")]
    pub create_ts: i32,
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "version_id")]
    pub version_id: String,
    #[serde(rename = "name_id")]
    pub name_id: String,
}

impl NamespaceSummary {
    pub fn new(namespace_id: String, create_ts: i32, display_name: String, version_id: String, name_id: String) -> NamespaceSummary {
        NamespaceSummary {
            namespace_id,
            create_ts,
            display_name,
            version_id,
            name_id,
        }
    }
}


