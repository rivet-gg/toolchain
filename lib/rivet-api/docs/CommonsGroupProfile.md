# CommonsGroupProfile

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**group_id** | **String** | A universally unique identifier. | 
**display_name** | **String** | Represent a resource's readable display name. | 
**avatar_url** | Option<**String**> | The URL of this group's avatar image. | [optional]
**external** | [**crate::models::CommonsGroupExternalLinks**](CommonsGroupExternalLinks.md) |  | 
**is_developer** | Option<**bool**> | Whether or not this group is a developer. | [optional]
**bio** | **String** | Detailed information about a profile. | 
**is_current_identity_member** | Option<**bool**> | Whether or not the current identity is a member of this group. | [optional]
**publicity** | [**crate::models::CommonsGroupPublicity**](CommonsGroupPublicity.md) |  | 
**member_count** | Option<**f64**> | Unsigned 32 bit integer. | [optional]
**members** | [**Vec<crate::models::CommonsGroupMember>**](CommonsGroupMember.md) | A list of group members. | 
**join_requests** | [**Vec<crate::models::CommonsGroupJoinRequest>**](CommonsGroupJoinRequest.md) | A list of group join requests. | 
**is_current_identity_requesting_join** | Option<**bool**> | Whether or not the current identity is currently requesting to join this group. | [optional]
**owner_identity_id** | **String** | A universally unique identifier. | 
**thread_id** | Option<**String**> | A universally unique identifier. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


