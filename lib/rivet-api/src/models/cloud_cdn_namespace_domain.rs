/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CloudCdnNamespaceDomain : A CDN domain for a given namespace.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CloudCdnNamespaceDomain {
    /// RFC3339 timestamp.
    #[serde(rename = "create_ts")]
    pub create_ts: String,
    /// A valid domain name (no protocol).
    #[serde(rename = "domain")]
    pub domain: String,
    #[serde(rename = "verification_errors")]
    pub verification_errors: Vec<String>,
    #[serde(rename = "verification_method")]
    pub verification_method: Box<crate::models::CloudCdnNamespaceDomainVerificationMethod>,
    #[serde(rename = "verification_status")]
    pub verification_status: crate::models::CloudCdnNamespaceDomainVerificationStatus,
}

impl CloudCdnNamespaceDomain {
    /// A CDN domain for a given namespace.
    pub fn new(create_ts: String, domain: String, verification_errors: Vec<String>, verification_method: crate::models::CloudCdnNamespaceDomainVerificationMethod, verification_status: crate::models::CloudCdnNamespaceDomainVerificationStatus) -> CloudCdnNamespaceDomain {
        CloudCdnNamespaceDomain {
            create_ts,
            domain,
            verification_errors,
            verification_method: Box::new(verification_method),
            verification_status,
        }
    }
}


