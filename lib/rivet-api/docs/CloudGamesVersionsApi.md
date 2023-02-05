# \CloudGamesVersionsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cloud_games_versions_create_game_version**](CloudGamesVersionsApi.md#cloud_games_versions_create_game_version) | **POST** /games/{game_id}/versions | 
[**cloud_games_versions_get_game_version_by_id**](CloudGamesVersionsApi.md#cloud_games_versions_get_game_version_by_id) | **GET** /games/{game_id}/versions/{version_id} | 
[**cloud_games_versions_validate_game_version**](CloudGamesVersionsApi.md#cloud_games_versions_validate_game_version) | **POST** /games/{game_id}/versions/validate | 



## cloud_games_versions_create_game_version

> crate::models::CloudGamesCreateGameVersionOutput cloud_games_versions_create_game_version(game_id, cloud_games_create_game_version_input)


Creates a new game version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | A universally unique identifier. | [required] |
**cloud_games_create_game_version_input** | [**CloudGamesCreateGameVersionInput**](CloudGamesCreateGameVersionInput.md) |  | [required] |

### Return type

[**crate::models::CloudGamesCreateGameVersionOutput**](CloudGamesCreateGameVersionOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_games_versions_get_game_version_by_id

> crate::models::CloudGamesGetGameVersionByIdOutput cloud_games_versions_get_game_version_by_id(game_id, version_id)


Returns a game version by its version ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | A universally unique identifier. | [required] |
**version_id** | **String** | A universally unique identifier. | [required] |

### Return type

[**crate::models::CloudGamesGetGameVersionByIdOutput**](CloudGamesGetGameVersionByIdOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_games_versions_validate_game_version

> crate::models::CloudGamesValidateGameVersionOutput cloud_games_versions_validate_game_version(game_id, cloud_games_validate_game_version_input)


Validates information used to create a new game version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | A universally unique identifier. | [required] |
**cloud_games_validate_game_version_input** | [**CloudGamesValidateGameVersionInput**](CloudGamesValidateGameVersionInput.md) |  | [required] |

### Return type

[**crate::models::CloudGamesValidateGameVersionOutput**](CloudGamesValidateGameVersionOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

