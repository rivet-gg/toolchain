# \GroupJoinRequestsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**join_requests_create_join_request**](GroupJoinRequestsApi.md#join_requests_create_join_request) | **POST** /groups/{group_id}/join-request | 
[**join_requests_resolve_join_request**](GroupJoinRequestsApi.md#join_requests_resolve_join_request) | **POST** /groups/{group_id}/join-request/{identity_id} | 



## join_requests_create_join_request

> join_requests_create_join_request(group_id)


Requests to join a group.

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


## join_requests_resolve_join_request

> join_requests_resolve_join_request(group_id, identity_id, group_resolve_join_request_input)


Resolves a join request for a given group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | A universally unique identifier. | [required] |
**identity_id** | **String** | A universally unique identifier. | [required] |
**group_resolve_join_request_input** | [**GroupResolveJoinRequestInput**](GroupResolveJoinRequestInput.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

