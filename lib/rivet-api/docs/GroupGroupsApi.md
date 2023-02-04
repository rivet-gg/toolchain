# \GroupGroupsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**groups_service_period_ban_group_identity**](GroupGroupsApi.md#groups_service_period_ban_group_identity) | **POST** /groups/{group_id}/bans/{identity_id} | 
[**groups_service_period_complete_group_avatar_upload**](GroupGroupsApi.md#groups_service_period_complete_group_avatar_upload) | **POST** /groups/{group_id}/avatar-upload/{upload_id}/complete | 
[**groups_service_period_create_group**](GroupGroupsApi.md#groups_service_period_create_group) | **POST** /groups | 
[**groups_service_period_get_group_bans**](GroupGroupsApi.md#groups_service_period_get_group_bans) | **GET** /groups/{group_id}/bans | 
[**groups_service_period_get_group_join_requests**](GroupGroupsApi.md#groups_service_period_get_group_join_requests) | **GET** /groups/{group_id}/join-requests | 
[**groups_service_period_get_group_members**](GroupGroupsApi.md#groups_service_period_get_group_members) | **GET** /groups/{group_id}/members | 
[**groups_service_period_get_group_profile**](GroupGroupsApi.md#groups_service_period_get_group_profile) | **GET** /groups/{group_id}/profile | 
[**groups_service_period_get_group_summary**](GroupGroupsApi.md#groups_service_period_get_group_summary) | **GET** /groups/{group_id}/summary | 
[**groups_service_period_kick_group_member**](GroupGroupsApi.md#groups_service_period_kick_group_member) | **POST** /groups/{group_id}/kick/{identity_id} | 
[**groups_service_period_leave_group**](GroupGroupsApi.md#groups_service_period_leave_group) | **POST** /groups/{group_id}/leave | 
[**groups_service_period_list_suggested_groups**](GroupGroupsApi.md#groups_service_period_list_suggested_groups) | **GET** /groups | 
[**groups_service_period_prepare_group_avatar_upload**](GroupGroupsApi.md#groups_service_period_prepare_group_avatar_upload) | **POST** /groups/avatar-upload/prepare | 
[**groups_service_period_search_groups**](GroupGroupsApi.md#groups_service_period_search_groups) | **GET** /groups/search | 
[**groups_service_period_transfer_group_ownership**](GroupGroupsApi.md#groups_service_period_transfer_group_ownership) | **POST** /groups/{group_id}/transfer-owner | 
[**groups_service_period_unban_group_identity**](GroupGroupsApi.md#groups_service_period_unban_group_identity) | **DELETE** /groups/{group_id}/bans/{identity_id} | 
[**groups_service_period_update_group_profile**](GroupGroupsApi.md#groups_service_period_update_group_profile) | **POST** /groups/{group_id}/profile | 
[**groups_service_period_validate_group_profile**](GroupGroupsApi.md#groups_service_period_validate_group_profile) | **POST** /groups/profile/validate | 



## groups_service_period_ban_group_identity

> groups_service_period_ban_group_identity(group_id, identity_id)


Bans an identity from a group. Must be the owner of the group to perform this action. The banned identity will no longer be able to create a join request or use a group invite.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | A universally unique identifier. | [required] |
**identity_id** | **String** | A universally unique identifier. | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_service_period_complete_group_avatar_upload

> groups_service_period_complete_group_avatar_upload(group_id, upload_id)


Completes an avatar image upload. Must be called after the file upload process completes. Call `rivet.api.group#PrepareGroupAvatarUpload` first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | A universally unique identifier. | [required] |
**upload_id** | **String** | A universally unique identifier. | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_service_period_create_group

> crate::models::GroupCreateGroupOutput groups_service_period_create_group(group_create_group_input)


Creates a new group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_create_group_input** | [**GroupCreateGroupInput**](GroupCreateGroupInput.md) |  | [required] |

### Return type

[**crate::models::GroupCreateGroupOutput**](GroupCreateGroupOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_service_period_get_group_bans

> crate::models::GroupGetGroupBansOutput groups_service_period_get_group_bans(group_id, anchor, count, watch_index)


Returns a group's bans. Must have valid permissions to view.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | A universally unique identifier. | [required] |
**anchor** | Option<**String**> | The pagination anchor. Set to the returned anchor of this endpoint to receive the next set of items. |  |
**count** | Option<**f64**> | Amount of bans to return. |  |
**watch_index** | Option<**String**> | A query parameter denoting the requests watch index. |  |

### Return type

[**crate::models::GroupGetGroupBansOutput**](GroupGetGroupBansOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_service_period_get_group_join_requests

> crate::models::GroupGetGroupJoinRequestsOutput groups_service_period_get_group_join_requests(group_id, anchor, count, watch_index)


Returns a group's join requests. Must have valid permissions to view.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | A universally unique identifier. | [required] |
**anchor** | Option<**String**> | The pagination anchor. Set to the returned anchor of this endpoint to receive the next set of items. |  |
**count** | Option<**f64**> | Amount of join requests to return. |  |
**watch_index** | Option<**String**> | A query parameter denoting the requests watch index. |  |

### Return type

[**crate::models::GroupGetGroupJoinRequestsOutput**](GroupGetGroupJoinRequestsOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_service_period_get_group_members

> crate::models::GroupGetGroupMembersOutput groups_service_period_get_group_members(group_id, anchor, count, watch_index)


Returns a group's members.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | A universally unique identifier. | [required] |
**anchor** | Option<**String**> | The pagination anchor. Set to the returned anchor of this endpoint to receive the next set of items. |  |
**count** | Option<**f64**> | Amount of members to return. |  |
**watch_index** | Option<**String**> | A query parameter denoting the requests watch index. |  |

### Return type

[**crate::models::GroupGetGroupMembersOutput**](GroupGetGroupMembersOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_service_period_get_group_profile

> crate::models::GroupGetGroupProfileOutput groups_service_period_get_group_profile(group_id, watch_index)


Returns a group profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | A universally unique identifier. | [required] |
**watch_index** | Option<**String**> | A query parameter denoting the requests watch index. |  |

### Return type

[**crate::models::GroupGetGroupProfileOutput**](GroupGetGroupProfileOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_service_period_get_group_summary

> crate::models::GroupGetGroupSummaryOutput groups_service_period_get_group_summary(group_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | A universally unique identifier. | [required] |

### Return type

[**crate::models::GroupGetGroupSummaryOutput**](GroupGetGroupSummaryOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_service_period_kick_group_member

> groups_service_period_kick_group_member(group_id, identity_id)


Kicks an identity from a group. Must be the owner of the group to perform this action.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | A universally unique identifier. | [required] |
**identity_id** | **String** | A universally unique identifier. | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_service_period_leave_group

> groups_service_period_leave_group(group_id)


Leaves a group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | A universally unique identifier. | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_service_period_list_suggested_groups

> crate::models::GroupListSuggestedGroupsOutput groups_service_period_list_suggested_groups(watch_index)


Returns a list of suggested groups.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**watch_index** | Option<**String**> | A query parameter denoting the requests watch index. |  |

### Return type

[**crate::models::GroupListSuggestedGroupsOutput**](GroupListSuggestedGroupsOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_service_period_prepare_group_avatar_upload

> crate::models::GroupPrepareGroupAvatarUploadOutput groups_service_period_prepare_group_avatar_upload(group_prepare_group_avatar_upload_input)


Prepares an avatar image upload. Complete upload with `rivet.api.group#CompleteGroupAvatarUpload`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_prepare_group_avatar_upload_input** | [**GroupPrepareGroupAvatarUploadInput**](GroupPrepareGroupAvatarUploadInput.md) |  | [required] |

### Return type

[**crate::models::GroupPrepareGroupAvatarUploadOutput**](GroupPrepareGroupAvatarUploadOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_service_period_search_groups

> crate::models::GroupSearchGroupsOutput groups_service_period_search_groups(query, anchor, limit)


Fuzzy search for groups.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | The query to match group display names against. | [required] |
**anchor** | Option<**String**> |  |  |
**limit** | Option<**f64**> | Unsigned 32 bit integer. |  |

### Return type

[**crate::models::GroupSearchGroupsOutput**](GroupSearchGroupsOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_service_period_transfer_group_ownership

> groups_service_period_transfer_group_ownership(group_id, group_transfer_group_ownership_input)


Transfers ownership of a group to another identity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | A universally unique identifier. | [required] |
**group_transfer_group_ownership_input** | [**GroupTransferGroupOwnershipInput**](GroupTransferGroupOwnershipInput.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_service_period_unban_group_identity

> groups_service_period_unban_group_identity(group_id, identity_id)


Unbans an identity from a group. Must be the owner of the group to perform this action.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | A universally unique identifier. | [required] |
**identity_id** | **String** | A universally unique identifier. | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_service_period_update_group_profile

> groups_service_period_update_group_profile(group_id, group_update_group_profile_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | A universally unique identifier. | [required] |
**group_update_group_profile_input** | [**GroupUpdateGroupProfileInput**](GroupUpdateGroupProfileInput.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_service_period_validate_group_profile

> crate::models::GroupValidateGroupProfileOutput groups_service_period_validate_group_profile(group_validate_group_profile_input)


Validate contents of group profile. Use to provide immediate feedback on profile changes before committing them.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_validate_group_profile_input** | [**GroupValidateGroupProfileInput**](GroupValidateGroupProfileInput.md) |  | [required] |

### Return type

[**crate::models::GroupValidateGroupProfileOutput**](GroupValidateGroupProfileOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

