/*
 * Rivet API EE
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ActorPortAuthorization {
    #[serde(rename = "bearer", skip_serializing_if = "Option::is_none")]
    pub bearer: Option<String>,
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<Box<crate::models::ActorPortQueryAuthorization>>,
}

impl ActorPortAuthorization {
    pub fn new() -> ActorPortAuthorization {
        ActorPortAuthorization {
            bearer: None,
            query: None,
        }
    }
}


