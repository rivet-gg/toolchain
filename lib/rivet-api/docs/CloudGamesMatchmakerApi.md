# \CloudGamesMatchmakerApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**matchmaker_delete_matchmaker_lobby**](CloudGamesMatchmakerApi.md#matchmaker_delete_matchmaker_lobby) | **DELETE** /games/{game_id}/matchmaker/lobbies/{lobby_id} | 
[**matchmaker_export_lobby_logs**](CloudGamesMatchmakerApi.md#matchmaker_export_lobby_logs) | **POST** /games/{game_id}/matchmaker/lobbies/{lobby_id}/logs/export | 
[**matchmaker_export_matchmaker_lobby_history**](CloudGamesMatchmakerApi.md#matchmaker_export_matchmaker_lobby_history) | **POST** /games/{game_id}/matchmaker/lobbies/export-history | 
[**matchmaker_get_lobby_logs**](CloudGamesMatchmakerApi.md#matchmaker_get_lobby_logs) | **GET** /games/{game_id}/matchmaker/lobbies/{lobby_id}/logs | 



## matchmaker_delete_matchmaker_lobby

> crate::models::CloudGamesDeleteMatchmakerLobbyOutput matchmaker_delete_matchmaker_lobby(game_id, lobby_id)


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


## matchmaker_export_lobby_logs

> crate::models::CloudGamesExportLobbyLogsOutput matchmaker_export_lobby_logs(game_id, lobby_id, cloud_games_export_lobby_logs_input)


Generates a download URL for logs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | A universally unique identifier. | [required] |
**lobby_id** | **String** | A universally unique identifier. | [required] |
**cloud_games_export_lobby_logs_input** | [**CloudGamesExportLobbyLogsInput**](CloudGamesExportLobbyLogsInput.md) |  | [required] |

### Return type

[**crate::models::CloudGamesExportLobbyLogsOutput**](CloudGamesExportLobbyLogsOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## matchmaker_export_matchmaker_lobby_history

> crate::models::CloudGamesExportMatchmakerLobbyHistoryOutput matchmaker_export_matchmaker_lobby_history(game_id, cloud_games_export_matchmaker_lobby_history_input)


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


## matchmaker_get_lobby_logs

> crate::models::CloudGamesGetLobbyLogsOutput matchmaker_get_lobby_logs(game_id, lobby_id, stream, watch_index)


Returns the logs for a given lobby.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | A universally unique identifier. | [required] |
**lobby_id** | **String** | A universally unique identifier. | [required] |
**stream** | **String** |  | [required] |
**watch_index** | Option<**String**> | A query parameter denoting the requests watch index. |  |

### Return type

[**crate::models::CloudGamesGetLobbyLogsOutput**](CloudGamesGetLobbyLogsOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

