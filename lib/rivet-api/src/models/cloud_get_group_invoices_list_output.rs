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
pub struct CloudGetGroupInvoicesListOutput {
    /// The pagination anchor.
    #[serde(rename = "anchor", skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    /// A list of a group's billing invoices.
    #[serde(rename = "invoices")]
    pub invoices: Vec<crate::models::CloudGroupBillingInvoice>,
}

impl CloudGetGroupInvoicesListOutput {
    pub fn new(invoices: Vec<crate::models::CloudGroupBillingInvoice>) -> CloudGetGroupInvoicesListOutput {
        CloudGetGroupInvoicesListOutput {
            anchor: None,
            invoices,
        }
    }
}


