# CommonsIdentityProfile

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**identity_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**display_name** | **String** |  | 
**account_number** | **i32** |  | 
**avatar_url** | **String** | The URL of this identity's avatar image. | 
**presence** | Option<[**crate::models::CommonsIdentityPresence**](CommonsIdentityPresence.md)> |  | [optional]
**party** | Option<[**crate::models::CommonsPartySummary**](CommonsPartySummary.md)> |  | [optional]
**is_registered** | **bool** | Whether or not this identity is registered with a linked account. | 
**external** | [**crate::models::CommonsIdentityExternalLinks**](CommonsIdentityExternalLinks.md) |  | 
**is_admin** | **bool** | Whether or not this identity is an admin. | 
**is_game_linked** | Option<**bool**> | Whether or not this game user has been linked through the Rivet dashboard. | [optional]
**dev_state** | Option<[**crate::models::CommonsIdentityDevState**](CommonsIdentityDevState.md)> |  | [optional]
**follower_count** | **i32** |  | 
**following_count** | **i32** |  | 
**following** | **bool** | Whether or not the requestee's identity is following this identity. | 
**is_following_me** | **bool** | Whether or not this identity is both followng and is followed by the requestee's identity. | 
**is_mutual_following** | **bool** |  | 
**join_ts** | **String** | RFC3339 timestamp | 
**bio** | **String** | Follows regex ^(?:[^\\n\\r]+\\n?|\\n){1,5}$ | 
**linked_accounts** | [**Vec<crate::models::CommonsIdentityLinkedAccount>**](CommonsIdentityLinkedAccount.md) |  | 
**groups** | [**Vec<crate::models::CommonsIdentityGroup>**](CommonsIdentityGroup.md) |  | 
**games** | [**Vec<crate::models::CommonsGameStatSummary>**](CommonsGameStatSummary.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


