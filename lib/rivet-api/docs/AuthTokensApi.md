# \AuthTokensApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**tokens_service_period_refresh_identity_token**](AuthTokensApi.md#tokens_service_period_refresh_identity_token) | **POST** /tokens/identity | 



## tokens_service_period_refresh_identity_token

> crate::models::AuthRefreshIdentityTokenOutput tokens_service_period_refresh_identity_token(auth_refresh_identity_token_input)


Refreshes the current identity's token and sets authentication headers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_refresh_identity_token_input** | [**AuthRefreshIdentityTokenInput**](AuthRefreshIdentityTokenInput.md) |  | [required] |

### Return type

[**crate::models::AuthRefreshIdentityTokenOutput**](AuthRefreshIdentityTokenOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

