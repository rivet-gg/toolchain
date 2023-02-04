# \MatchmakerLobbiesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**lobbies_service_period_find**](MatchmakerLobbiesApi.md#lobbies_service_period_find) | **POST** /lobbies/find | 
[**lobbies_service_period_join**](MatchmakerLobbiesApi.md#lobbies_service_period_join) | **POST** /lobbies/join | 
[**lobbies_service_period_list**](MatchmakerLobbiesApi.md#lobbies_service_period_list) | **GET** /lobbies/list | 
[**lobbies_service_period_ready**](MatchmakerLobbiesApi.md#lobbies_service_period_ready) | **POST** /lobbies/ready | 
[**lobbies_service_period_set_closed**](MatchmakerLobbiesApi.md#lobbies_service_period_set_closed) | **PUT** /lobbies/closed | 



## lobbies_service_period_find

> crate::models::MatchmakerFindLobbyOutput lobbies_service_period_find(lobbies_service_find_request, origin)


Finds a lobby based on the given criteria. If a lobby is not found and `prevent_auto_create_lobby` is `true`,  a new lobby will be created. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lobbies_service_find_request** | [**LobbiesServiceFindRequest**](LobbiesServiceFindRequest.md) |  | [required] |
**origin** | Option<**String**> |  |  |

### Return type

[**crate::models::MatchmakerFindLobbyOutput**](MatchmakerFindLobbyOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lobbies_service_period_join

> crate::models::MatchmakerJoinLobbyOutput lobbies_service_period_join(lobbies_service_join_request)


Joins a specific lobby. This request will use the direct player count configured for the lobby group. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lobbies_service_join_request** | [**LobbiesServiceJoinRequest**](LobbiesServiceJoinRequest.md) |  | [required] |

### Return type

[**crate::models::MatchmakerJoinLobbyOutput**](MatchmakerJoinLobbyOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lobbies_service_period_list

> crate::models::MatchmakerListLobbiesOutput lobbies_service_period_list()


Lists all open lobbies.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::MatchmakerListLobbiesOutput**](MatchmakerListLobbiesOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lobbies_service_period_ready

> lobbies_service_period_ready()


Marks the current lobby as ready to accept connections.  Players will not be able to connect to this lobby until the  lobby is flagged as ready.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lobbies_service_period_set_closed

> lobbies_service_period_set_closed(lobbies_service_set_closed_request)


If `is_closed` is `true`, players will be prevented from joining the lobby. Does not shutdown the lobby. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lobbies_service_set_closed_request** | [**LobbiesServiceSetClosedRequest**](LobbiesServiceSetClosedRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

