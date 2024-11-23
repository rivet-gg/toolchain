/*
 * Rivet API EE
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CloudBootstrapAccess {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "development")]
    Development,

}

impl ToString for CloudBootstrapAccess {
    fn to_string(&self) -> String {
        match self {
            Self::Public => String::from("public"),
            Self::Private => String::from("private"),
            Self::Development => String::from("development"),
        }
    }
}

impl Default for CloudBootstrapAccess {
    fn default() -> CloudBootstrapAccess {
        Self::Public
    }
}




