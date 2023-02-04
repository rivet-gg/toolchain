# CommonsGroupSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**group_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**dispay_name** | **String** |  | 
**avatar_url** | Option<**String**> | The URL of this group's avatar image. | [optional]
**external** | [**crate::models::CommonsGroupExternalLinks**](CommonsGroupExternalLinks.md) |  | 
**is_developer** | **bool** | Whether or not this group is a developer. | 
**bio** | **String** | Follows regex ^(?:[^\\n\\r]+\\n?|\\n){1,5}$ | 
**is_currently_identity_member** | **bool** | Whether or not the current identity is a member of this group. | 
**publicity** | [**crate::models::CommonsGroupPublicity**](CommonsGroupPublicity.md) |  | 
**member_count** | **i32** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


