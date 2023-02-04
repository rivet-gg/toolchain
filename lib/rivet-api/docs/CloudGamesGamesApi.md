# \CloudGamesGamesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**games_service_period_create_game**](CloudGamesGamesApi.md#games_service_period_create_game) | **POST** /games | 
[**games_service_period_game_banner_upload_complete**](CloudGamesGamesApi.md#games_service_period_game_banner_upload_complete) | **POST** /games/{game_id}/banner-upload/{upload_id}/complete | 
[**games_service_period_game_banner_upload_prepare**](CloudGamesGamesApi.md#games_service_period_game_banner_upload_prepare) | **POST** /games/{game_id}/banner-upload/prepare | 
[**games_service_period_game_logo_upload_complete**](CloudGamesGamesApi.md#games_service_period_game_logo_upload_complete) | **POST** /games/{game_id}/logo-upload/{upload_id}/complete | 
[**games_service_period_game_logo_upload_prepare**](CloudGamesGamesApi.md#games_service_period_game_logo_upload_prepare) | **POST** /games/{game_id}/logo-upload/prepare | 
[**games_service_period_get_game_by_id**](CloudGamesGamesApi.md#games_service_period_get_game_by_id) | **GET** /games/{game_id} | 
[**games_service_period_get_games**](CloudGamesGamesApi.md#games_service_period_get_games) | **GET** /games | 
[**games_service_period_validate_game**](CloudGamesGamesApi.md#games_service_period_validate_game) | **POST** /games/validate | 



## games_service_period_create_game

> crate::models::CloudGamesCreateGameOutput games_service_period_create_game(cloud_games_create_game_input)


Creates a new game.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cloud_games_create_game_input** | [**CloudGamesCreateGameInput**](CloudGamesCreateGameInput.md) |  | [required] |

### Return type

[**crate::models::CloudGamesCreateGameOutput**](CloudGamesCreateGameOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## games_service_period_game_banner_upload_complete

> games_service_period_game_banner_upload_complete(game_id, upload_id)


Completes an game banner image upload. Must be called after the file upload process completes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | A universally unique identifier. | [required] |
**upload_id** | **String** | A universally unique identifier. | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## games_service_period_game_banner_upload_prepare

> crate::models::CloudGamesGameBannerUploadPrepareOutput games_service_period_game_banner_upload_prepare(game_id, cloud_games_game_banner_upload_prepare_input)


Prepares a game banner image upload.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | A universally unique identifier. | [required] |
**cloud_games_game_banner_upload_prepare_input** | [**CloudGamesGameBannerUploadPrepareInput**](CloudGamesGameBannerUploadPrepareInput.md) |  | [required] |

### Return type

[**crate::models::CloudGamesGameBannerUploadPrepareOutput**](CloudGamesGameBannerUploadPrepareOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## games_service_period_game_logo_upload_complete

> games_service_period_game_logo_upload_complete(game_id, upload_id)


Completes a game logo image upload. Must be called after the file upload process completes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | A universally unique identifier. | [required] |
**upload_id** | **String** | A universally unique identifier. | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## games_service_period_game_logo_upload_prepare

> crate::models::CloudGamesGameLogoUploadPrepareOutput games_service_period_game_logo_upload_prepare(game_id, cloud_games_game_logo_upload_prepare_input)


Prepares a game logo image upload.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | A universally unique identifier. | [required] |
**cloud_games_game_logo_upload_prepare_input** | [**CloudGamesGameLogoUploadPrepareInput**](CloudGamesGameLogoUploadPrepareInput.md) |  | [required] |

### Return type

[**crate::models::CloudGamesGameLogoUploadPrepareOutput**](CloudGamesGameLogoUploadPrepareOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## games_service_period_get_game_by_id

> crate::models::CloudGamesGetGameByIdOutput games_service_period_get_game_by_id(game_id, watch_index)


Returns a game by its game id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | A universally unique identifier. | [required] |
**watch_index** | Option<**String**> | A query parameter denoting the requests watch index. |  |

### Return type

[**crate::models::CloudGamesGetGameByIdOutput**](CloudGamesGetGameByIdOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## games_service_period_get_games

> crate::models::CloudGamesGetGamesOutput games_service_period_get_games(watch_index)


Returns a list of games in which the current identity is a group member of its development team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**watch_index** | Option<**String**> | A query parameter denoting the requests watch index. |  |

### Return type

[**crate::models::CloudGamesGetGamesOutput**](CloudGamesGetGamesOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## games_service_period_validate_game

> crate::models::CloudGamesValidateGameOutput games_service_period_validate_game(cloud_games_validate_game_input)


Validates information used to create a new game.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cloud_games_validate_game_input** | [**CloudGamesValidateGameInput**](CloudGamesValidateGameInput.md) |  | [required] |

### Return type

[**crate::models::CloudGamesValidateGameOutput**](CloudGamesValidateGameOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

