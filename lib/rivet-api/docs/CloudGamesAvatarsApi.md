# \CloudGamesAvatarsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**avatars_service_period_complete_custom_avatar_upload**](CloudGamesAvatarsApi.md#avatars_service_period_complete_custom_avatar_upload) | **POST** /games/{game_id}/avatar-upload/{upload_id}/complete | 
[**avatars_service_period_list_game_custom_avatars**](CloudGamesAvatarsApi.md#avatars_service_period_list_game_custom_avatars) | **GET** /games/{game_id}/avatars | 
[**avatars_service_period_prepare_custom_avatar_upload**](CloudGamesAvatarsApi.md#avatars_service_period_prepare_custom_avatar_upload) | **POST** /games/{game_id}/prepare | 



## avatars_service_period_complete_custom_avatar_upload

> avatars_service_period_complete_custom_avatar_upload(game_id, upload_id)


Completes a custom avatar image upload. Must be called after the file upload process completes.

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


## avatars_service_period_list_game_custom_avatars

> crate::models::CloudGamesListGameCustomAvatarsOutput avatars_service_period_list_game_custom_avatars(game_id)


Lists custom avatars for the given game.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | A universally unique identifier. | [required] |

### Return type

[**crate::models::CloudGamesListGameCustomAvatarsOutput**](CloudGamesListGameCustomAvatarsOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## avatars_service_period_prepare_custom_avatar_upload

> crate::models::CloudGamesPrepareCustomAvatarUploadOutput avatars_service_period_prepare_custom_avatar_upload(game_id, cloud_games_prepare_custom_avatar_upload_input)


Prepares a custom avatar image upload. Complete upload with `rivet.api.cloud#CompleteCustomAvatarUpload`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | A universally unique identifier. | [required] |
**cloud_games_prepare_custom_avatar_upload_input** | [**CloudGamesPrepareCustomAvatarUploadInput**](CloudGamesPrepareCustomAvatarUploadInput.md) |  | [required] |

### Return type

[**crate::models::CloudGamesPrepareCustomAvatarUploadOutput**](CloudGamesPrepareCustomAvatarUploadOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

