# CommonsIdentityHandle

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**identity_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**display_name** | **String** |  | 
**account_number** | **i32** |  | 
**avatar_url** | **String** | The URL of this identity's avatar image. | 
**presence** | Option<[**crate::models::CommonsIdentityPresence**](CommonsIdentityPresence.md)> |  | [optional]
**party** | Option<[**crate::models::CommonsPartyHandle**](CommonsPartyHandle.md)> |  | [optional]
**is_registered** | **bool** | Whether or not this identity is registered with a linked account. | 
**external** | [**crate::models::CommonsIdentityExternalLinks**](CommonsIdentityExternalLinks.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


