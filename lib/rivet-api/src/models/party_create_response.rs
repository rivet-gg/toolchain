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
pub struct PartyCreateResponse {
    #[serde(rename = "invites")]
    pub invites: Vec<crate::models::PartyCreatedInvite>,
    #[serde(rename = "party_id")]
    pub party_id: uuid::Uuid,
}

impl PartyCreateResponse {
    pub fn new(invites: Vec<crate::models::PartyCreatedInvite>, party_id: uuid::Uuid) -> PartyCreateResponse {
        PartyCreateResponse {
            invites,
            party_id,
        }
    }
}


