# \CloudGamesNamespacesAnalyticsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cloud_games_namespaces_analytics_get_namespace_analytics_matchmaker_live**](CloudGamesNamespacesAnalyticsApi.md#cloud_games_namespaces_analytics_get_namespace_analytics_matchmaker_live) | **GET** /games/{game_id}/namespaces/{namespace_id}/analytics/matchmaker/live | 



## cloud_games_namespaces_analytics_get_namespace_analytics_matchmaker_live

> crate::models::CloudGamesNamespacesGetNamespaceAnalyticsMatchmakerLiveOutput cloud_games_namespaces_analytics_get_namespace_analytics_matchmaker_live(game_id, namespace_id)


Returns live information about all active lobies for a given namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | A universally unique identifier. | [required] |
**namespace_id** | **String** | A universally unique identifier. | [required] |

### Return type

[**crate::models::CloudGamesNamespacesGetNamespaceAnalyticsMatchmakerLiveOutput**](CloudGamesNamespacesGetNamespaceAnalyticsMatchmakerLiveOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

