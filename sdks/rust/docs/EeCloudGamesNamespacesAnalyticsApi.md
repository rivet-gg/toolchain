# \EeCloudGamesNamespacesAnalyticsApi

All URIs are relative to *https://api.rivet.gg*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ee_cloud_games_namespaces_analytics_get_analytics**](EeCloudGamesNamespacesAnalyticsApi.md#ee_cloud_games_namespaces_analytics_get_analytics) | **GET** /cloud/games/namespaces/analytics | 



## ee_cloud_games_namespaces_analytics_get_analytics

> crate::models::EeCloudGamesNamespacesGetAnalyticsResponse ee_cloud_games_namespaces_analytics_get_analytics(query_start, query_end, game_ids, namespace_ids, variants)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query_start** | **String** |  | [required] |
**query_end** | **String** |  | [required] |
**game_ids** | **String** |  | [required] |
**namespace_ids** | **String** |  | [required] |
**variants** | [**EeCloudAnalyticsVariantQuery**](.md) |  | [required] |

### Return type

[**crate::models::EeCloudGamesNamespacesGetAnalyticsResponse**](EeCloudGamesNamespacesGetAnalyticsResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

