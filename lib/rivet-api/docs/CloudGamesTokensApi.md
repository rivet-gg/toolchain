# \CloudGamesTokensApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cloud_games_tokens_create_cloud_token**](CloudGamesTokensApi.md#cloud_games_tokens_create_cloud_token) | **POST** /games/{game_id}/tokens/cloud | 



## cloud_games_tokens_create_cloud_token

> crate::models::CloudGamesCreateCloudTokenOutput cloud_games_tokens_create_cloud_token(game_id)


Creates a new game cloud token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | A universally unique identifier. | [required] |

### Return type

[**crate::models::CloudGamesCreateCloudTokenOutput**](CloudGamesCreateCloudTokenOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

