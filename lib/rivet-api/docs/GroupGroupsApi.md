# \GroupGroupsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**groups_ban_identity**](GroupGroupsApi.md#groups_ban_identity) | **POST** /groups/{group_id}/bans/{identity_id} | 
[**groups_complete_avatar_upload**](GroupGroupsApi.md#groups_complete_avatar_upload) | **POST** /groups/{group_id}/avatar-upload/{upload_id}/complete | 
[**groups_create**](GroupGroupsApi.md#groups_create) | **POST** /groups | 
[**groups_get_bans**](GroupGroupsApi.md#groups_get_bans) | **GET** /groups/{group_id}/bans | 
[**groups_get_join_requests**](GroupGroupsApi.md#groups_get_join_requests) | **GET** /groups/{group_id}/join-requests | 
[**groups_get_members**](GroupGroupsApi.md#groups_get_members) | **GET** /groups/{group_id}/members | 
[**groups_get_profile**](GroupGroupsApi.md#groups_get_profile) | **GET** /groups/{group_id}/profile | 
[**groups_get_summary**](GroupGroupsApi.md#groups_get_summary) | **GET** /groups/{group_id}/summary | 
[**groups_kick_member**](GroupGroupsApi.md#groups_kick_member) | **POST** /groups/{group_id}/kick/{identity_id} | 
[**groups_leave**](GroupGroupsApi.md#groups_leave) | **POST** /groups/{group_id}/leave | 
[**groups_list_suggested**](GroupGroupsApi.md#groups_list_suggested) | **GET** /groups | 
[**groups_prepare_avatar_upload**](GroupGroupsApi.md#groups_prepare_avatar_upload) | **POST** /groups/avatar-upload/prepare | 
[**groups_search**](GroupGroupsApi.md#groups_search) | **GET** /groups/search | 
[**groups_transfer_ownership**](GroupGroupsApi.md#groups_transfer_ownership) | **POST** /groups/{group_id}/transfer-owner | 
[**groups_unban_identity**](GroupGroupsApi.md#groups_unban_identity) | **DELETE** /groups/{group_id}/bans/{identity_id} | 
[**groups_update_profile**](GroupGroupsApi.md#groups_update_profile) | **POST** /groups/{group_id}/profile | 
[**groups_validate_profile**](GroupGroupsApi.md#groups_validate_profile) | **POST** /groups/profile/validate | 



## groups_ban_identity

> groups_ban_identity(group_id, identity_id)


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


## groups_complete_avatar_upload

> groups_complete_avatar_upload(group_id, upload_id)


Completes an avatar image upload. Must be called after the file upload process completes. Call `rivet.api.group#PrepareAvatarUpload` first.

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


## groups_create

> crate::models::GroupCreateOutput groups_create(group_create_input)


Creates a new group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_create_input** | [**GroupCreateInput**](GroupCreateInput.md) |  | [required] |

### Return type

[**crate::models::GroupCreateOutput**](GroupCreateOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_get_bans

> crate::models::GroupGetBansOutput groups_get_bans(group_id, anchor, count, watch_index)


Returns a group's bans. Must have valid permissions to view.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | A universally unique identifier. | [required] |
**anchor** | Option<**String**> | The pagination anchor. Set to the returned anchor of this endpoint to receive the next set of items. |  |
**count** | Option<**f64**> | Amount of bans to return. |  |
**watch_index** | Option<**String**> | A query parameter denoting the requests watch index. |  |

### Return type

[**crate::models::GroupGetBansOutput**](GroupGetBansOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_get_join_requests

> crate::models::GroupGetJoinRequestsOutput groups_get_join_requests(group_id, anchor, count, watch_index)


Returns a group's join requests. Must have valid permissions to view.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | A universally unique identifier. | [required] |
**anchor** | Option<**String**> | The pagination anchor. Set to the returned anchor of this endpoint to receive the next set of items. |  |
**count** | Option<**f64**> | Amount of join requests to return. |  |
**watch_index** | Option<**String**> | A query parameter denoting the requests watch index. |  |

### Return type

[**crate::models::GroupGetJoinRequestsOutput**](GroupGetJoinRequestsOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_get_members

> crate::models::GroupGetMembersOutput groups_get_members(group_id, anchor, count, watch_index)


Returns a group's members.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | A universally unique identifier. | [required] |
**anchor** | Option<**String**> | The pagination anchor. Set to the returned anchor of this endpoint to receive the next set of items. |  |
**count** | Option<**f64**> | Amount of members to return. |  |
**watch_index** | Option<**String**> | A query parameter denoting the requests watch index. |  |

### Return type

[**crate::models::GroupGetMembersOutput**](GroupGetMembersOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_get_profile

> crate::models::GroupGetProfileOutput groups_get_profile(group_id, watch_index)


Returns a group profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | A universally unique identifier. | [required] |
**watch_index** | Option<**String**> | A query parameter denoting the requests watch index. |  |

### Return type

[**crate::models::GroupGetProfileOutput**](GroupGetProfileOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_get_summary

> crate::models::GroupGetSummaryOutput groups_get_summary(group_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | A universally unique identifier. | [required] |

### Return type

[**crate::models::GroupGetSummaryOutput**](GroupGetSummaryOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_kick_member

> groups_kick_member(group_id, identity_id)


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


## groups_leave

> groups_leave(group_id)


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


## groups_list_suggested

> crate::models::GroupListSuggestedOutput groups_list_suggested(watch_index)


Returns a list of suggested groups.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**watch_index** | Option<**String**> | A query parameter denoting the requests watch index. |  |

### Return type

[**crate::models::GroupListSuggestedOutput**](GroupListSuggestedOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_prepare_avatar_upload

> crate::models::GroupPrepareAvatarUploadOutput groups_prepare_avatar_upload(group_prepare_avatar_upload_input)


Prepares an avatar image upload. Complete upload with `rivet.api.group#CompleteAvatarUpload`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_prepare_avatar_upload_input** | [**GroupPrepareAvatarUploadInput**](GroupPrepareAvatarUploadInput.md) |  | [required] |

### Return type

[**crate::models::GroupPrepareAvatarUploadOutput**](GroupPrepareAvatarUploadOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_search

> crate::models::GroupSearchOutput groups_search(query, anchor, limit)


Fuzzy search for groups.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | The query to match group display names against. | [required] |
**anchor** | Option<**String**> |  |  |
**limit** | Option<**f64**> | Unsigned 32 bit integer. |  |

### Return type

[**crate::models::GroupSearchOutput**](GroupSearchOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_transfer_ownership

> groups_transfer_ownership(group_id, group_transfer_ownership_input)


Transfers ownership of a group to another identity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | A universally unique identifier. | [required] |
**group_transfer_ownership_input** | [**GroupTransferOwnershipInput**](GroupTransferOwnershipInput.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_unban_identity

> groups_unban_identity(group_id, identity_id)


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


## groups_update_profile

> groups_update_profile(group_id, group_update_profile_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | A universally unique identifier. | [required] |
**group_update_profile_input** | [**GroupUpdateProfileInput**](GroupUpdateProfileInput.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_validate_profile

> crate::models::GroupValidateProfileOutput groups_validate_profile(group_validate_profile_input)


Validate contents of group profile. Use to provide immediate feedback on profile changes before committing them.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_validate_profile_input** | [**GroupValidateProfileInput**](GroupValidateProfileInput.md) |  | [required] |

### Return type

[**crate::models::GroupValidateProfileOutput**](GroupValidateProfileOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

