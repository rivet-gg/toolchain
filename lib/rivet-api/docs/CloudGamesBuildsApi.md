# \CloudGamesBuildsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**builds_create_game_build**](CloudGamesBuildsApi.md#builds_create_game_build) | **POST** /games/{game_id}/builds | 
[**builds_list_game_builds**](CloudGamesBuildsApi.md#builds_list_game_builds) | **GET** /games/{game_id}/builds | 



## builds_create_game_build

> crate::models::CloudGamesCreateGameBuildOutput builds_create_game_build(game_id, cloud_games_create_game_build_input)


Creates a new game build for the given game.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | A universally unique identifier. | [required] |
**cloud_games_create_game_build_input** | [**CloudGamesCreateGameBuildInput**](CloudGamesCreateGameBuildInput.md) |  | [required] |

### Return type

[**crate::models::CloudGamesCreateGameBuildOutput**](CloudGamesCreateGameBuildOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## builds_list_game_builds

> crate::models::CloudGamesListGameBuildsOutput builds_list_game_builds(game_id)


Lists game builds for the given game.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | A universally unique identifier. | [required] |

### Return type

[**crate::models::CloudGamesListGameBuildsOutput**](CloudGamesListGameBuildsOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

