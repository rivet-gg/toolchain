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
pub struct ChatMessageBodyGroupMemberKick {
    #[serde(rename = "identity")]
    pub identity: Box<crate::models::IdentityHandle>,
}

impl ChatMessageBodyGroupMemberKick {
    pub fn new(identity: crate::models::IdentityHandle) -> ChatMessageBodyGroupMemberKick {
        ChatMessageBodyGroupMemberKick {
            identity: Box::new(identity),
        }
    }
}


