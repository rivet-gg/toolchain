# CloudGroupBillingPayment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount** | Option<**f64**> | Payment amount (in hundreths USD, 100 = $1.00). | [optional]
**created_ts** | **String** | RFC3339 timestamp. | 
**description** | Option<**String**> | A description of this payment. | [optional]
**from_invoice** | Option<**bool**> | Whether or not this payment is from an invoice. | [optional]
**status** | [**crate::models::CloudGroupBillingStatus**](CloudGroupBillingStatus.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


