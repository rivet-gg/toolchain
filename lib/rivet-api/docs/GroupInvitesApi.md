# \GroupInvitesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**invites_service_period_consume_group_invite**](GroupInvitesApi.md#invites_service_period_consume_group_invite) | **POST** /invites/{group_invite_code}/consume | 
[**invites_service_period_create_group_invite**](GroupInvitesApi.md#invites_service_period_create_group_invite) | **POST** /groups/{group_id}/invites | 
[**invites_service_period_get_group_invite**](GroupInvitesApi.md#invites_service_period_get_group_invite) | **GET** /invites/{group_invite_code} | 



## invites_service_period_consume_group_invite

> crate::models::GroupConsumeGroupInviteOutput invites_service_period_consume_group_invite(group_invite_code)


Consumes a group invite to join a group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_invite_code** | **String** | Provided by `rivet.api.group#CreateGroupInviteOutput$code`. | [required] |

### Return type

[**crate::models::GroupConsumeGroupInviteOutput**](GroupConsumeGroupInviteOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invites_service_period_create_group_invite

> crate::models::GroupCreateGroupInviteOutput invites_service_period_create_group_invite(group_id, group_create_group_invite_input)


Creates a group invite. Can be shared with other identities to let them join this group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | A universally unique identifier. | [required] |
**group_create_group_invite_input** | [**GroupCreateGroupInviteInput**](GroupCreateGroupInviteInput.md) |  | [required] |

### Return type

[**crate::models::GroupCreateGroupInviteOutput**](GroupCreateGroupInviteOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invites_service_period_get_group_invite

> crate::models::GroupGetGroupInviteOutput invites_service_period_get_group_invite(group_invite_code)


Inspects a group invite returning information about the team that created it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_invite_code** | **String** | Provided by `rivet.api.group#CreateGroupInviteOutput$code`. | [required] |

### Return type

[**crate::models::GroupGetGroupInviteOutput**](GroupGetGroupInviteOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

