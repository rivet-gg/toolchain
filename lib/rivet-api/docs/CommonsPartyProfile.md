# CommonsPartyProfile

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**party_id** | **String** | A universally unique identifier. | 
**create_ts** | **String** | RFC3339 timestamp. | 
**activity** | [**crate::models::CommonsPartyActivity**](CommonsPartyActivity.md) |  | 
**external** | [**crate::models::CommonsPartyExternalLinks**](CommonsPartyExternalLinks.md) |  | 
**publicity** | [**crate::models::CommonsPartyPublicity**](CommonsPartyPublicity.md) |  | 
**party_size** | Option<**f64**> | Unsigned 32 bit integer. | [optional]
**members** | [**Vec<crate::models::CommonsPartyMemberSummary>**](CommonsPartyMemberSummary.md) | A list of party members. | 
**thread_id** | **String** | A universally unique identifier. | 
**invites** | [**Vec<crate::models::CommonsPartyInvite>**](CommonsPartyInvite.md) | A list of party invites. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


