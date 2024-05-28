# \EeCloudGamesProjectsApi

All URIs are relative to *https://api.rivet.gg*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ee_cloud_games_projects_get**](EeCloudGamesProjectsApi.md#ee_cloud_games_projects_get) | **GET** /cloud/games/{game_id}/project | 
[**ee_cloud_games_projects_link**](EeCloudGamesProjectsApi.md#ee_cloud_games_projects_link) | **POST** /cloud/games/{game_id}/project-link | 



## ee_cloud_games_projects_get

> crate::models::EeCloudGamesProjectsGetResponse ee_cloud_games_projects_get(game_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::EeCloudGamesProjectsGetResponse**](EeCloudGamesProjectsGetResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_cloud_games_projects_link

> ee_cloud_games_projects_link(game_id, ee_cloud_games_projects_link_request)


Links a game to an OpenGB project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **uuid::Uuid** |  | [required] |
**ee_cloud_games_projects_link_request** | [**EeCloudGamesProjectsLinkRequest**](EeCloudGamesProjectsLinkRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

