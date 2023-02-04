# \CloudGamesMatchmakerApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**matchmaker_service_period_delete_matchmaker_lobby**](CloudGamesMatchmakerApi.md#matchmaker_service_period_delete_matchmaker_lobby) | **DELETE** /games/{game_id}/matchmaker/lobbies/{lobby_id} | 
[**matchmaker_service_period_export_matchmaker_lobby_history**](CloudGamesMatchmakerApi.md#matchmaker_service_period_export_matchmaker_lobby_history) | **POST** /games/{game_id}/matchmaker/lobbies/export-history | 



## matchmaker_service_period_delete_matchmaker_lobby

> crate::models::CloudGamesDeleteMatchmakerLobbyOutput matchmaker_service_period_delete_matchmaker_lobby(game_id, lobby_id)


Deletes a matchmaker lobby, stopping it immediately.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | A universally unique identifier. | [required] |
**lobby_id** | **String** | A universally unique identifier. | [required] |

### Return type

[**crate::models::CloudGamesDeleteMatchmakerLobbyOutput**](CloudGamesDeleteMatchmakerLobbyOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## matchmaker_service_period_export_matchmaker_lobby_history

> crate::models::CloudGamesExportMatchmakerLobbyHistoryOutput matchmaker_service_period_export_matchmaker_lobby_history(game_id, cloud_games_export_matchmaker_lobby_history_input)


Exports lobby history over a given query time span.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | A universally unique identifier. | [required] |
**cloud_games_export_matchmaker_lobby_history_input** | [**CloudGamesExportMatchmakerLobbyHistoryInput**](CloudGamesExportMatchmakerLobbyHistoryInput.md) |  | [required] |

### Return type

[**crate::models::CloudGamesExportMatchmakerLobbyHistoryOutput**](CloudGamesExportMatchmakerLobbyHistoryOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

