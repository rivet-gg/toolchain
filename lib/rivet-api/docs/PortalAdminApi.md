# \PortalAdminApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**portal_admin_resolve_beta_join_request**](PortalAdminApi.md#portal_admin_resolve_beta_join_request) | **POST** /beta-join-request/{identity_id} | 



## portal_admin_resolve_beta_join_request

> portal_admin_resolve_beta_join_request(identity_id, portal_resolve_beta_join_request_input)


Resolves a beta join request for a given identity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_id** | **String** | A universally unique identifier. | [required] |
**portal_resolve_beta_join_request_input** | [**PortalResolveBetaJoinRequestInput**](PortalResolveBetaJoinRequestInput.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

