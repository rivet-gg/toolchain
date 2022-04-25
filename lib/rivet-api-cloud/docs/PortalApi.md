# \PortalApi

All URIs are relative to *https://cloud.api.rivet.gg/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**complete_upload**](PortalApi.md#complete_upload) | **POST** /uploads/{upload_id}/complete | 
[**convert_team**](PortalApi.md#convert_team) | **POST** /teams/{team_id}/convert | 
[**create_cloud_token**](PortalApi.md#create_cloud_token) | **POST** /games/{game_id}/tokens/cloud | 
[**create_game**](PortalApi.md#create_game) | **POST** /games | 
[**create_game_build**](PortalApi.md#create_game_build) | **POST** /games/{game_id}/builds | 
[**create_game_cdn_site**](PortalApi.md#create_game_cdn_site) | **POST** /games/{game_id}/cdn/sites | 
[**create_game_namespace**](PortalApi.md#create_game_namespace) | **POST** /games/{game_id}/namespaces | 
[**create_game_namespace_token_development**](PortalApi.md#create_game_namespace_token_development) | **POST** /games/{game_id}/namespaces/{namespace_id}/tokens/development | 
[**create_game_namespace_token_public**](PortalApi.md#create_game_namespace_token_public) | **POST** /games/{game_id}/namespaces/{namespace_id}/tokens/public | 
[**create_game_version**](PortalApi.md#create_game_version) | **POST** /games/{game_id}/versions | 
[**delete_matchmaker_lobby**](PortalApi.md#delete_matchmaker_lobby) | **DELETE** /games/{game_id}/matchmaker/lobbies/{lobby_id} | 
[**export_matchmaker_lobby_history**](PortalApi.md#export_matchmaker_lobby_history) | **POST** /games/{game_id}/matchmaker/lobbies/export-history | 
[**get_game_by_id**](PortalApi.md#get_game_by_id) | **GET** /games/{game_id} | 
[**get_game_namespace_by_id**](PortalApi.md#get_game_namespace_by_id) | **GET** /games/{game_id}/namespaces/{namespace_id} | 
[**get_game_version_by_id**](PortalApi.md#get_game_version_by_id) | **GET** /games/{game_id}/versions/{version_id} | 
[**get_games**](PortalApi.md#get_games) | **GET** /games | 
[**get_namespace_analytics_matchmaker_live**](PortalApi.md#get_namespace_analytics_matchmaker_live) | **GET** /games/{game_id}/namespaces/{namespace_id}/analytics/matchmaker/live | 
[**get_namespace_lobby**](PortalApi.md#get_namespace_lobby) | **GET** /games/{game_id}/namespaces/{namespace_id}/logs/lobbies/{lobby_id} | 
[**get_ray_perf_logs**](PortalApi.md#get_ray_perf_logs) | **GET** /rays/{ray_id}/perf | 
[**get_region_tiers**](PortalApi.md#get_region_tiers) | **GET** /region-tiers | 
[**get_team_bank_transfers_list**](PortalApi.md#get_team_bank_transfers_list) | **GET** /teams/{team_id}/billing/transfers | 
[**get_team_billing**](PortalApi.md#get_team_billing) | **GET** /teams/{team_id}/billing | 
[**get_team_payments_list**](PortalApi.md#get_team_payments_list) | **GET** /teams/{team_id}/billing/payments | 
[**inspect**](PortalApi.md#inspect) | **GET** /auth/inspect | 
[**list_cdn_sites**](PortalApi.md#list_cdn_sites) | **GET** /games/{game_id}/cdn/sites | 
[**list_game_builds**](PortalApi.md#list_game_builds) | **GET** /games/{game_id}/builds | 
[**list_namespace_lobbies**](PortalApi.md#list_namespace_lobbies) | **GET** /games/{game_id}/namespaces/{namespace_id}/logs/lobbies | 
[**remove_namespace_domain**](PortalApi.md#remove_namespace_domain) | **DELETE** /games/{game_id}/namespaces/{namespace_id}/domains/{domain} | 
[**team_billing_checkout**](PortalApi.md#team_billing_checkout) | **POST** /teams/{team_id}/checkout | 
[**toggle_namespace_domain_public_auth**](PortalApi.md#toggle_namespace_domain_public_auth) | **PUT** /games/{game_id}/namespaces/{namespace_id}/domain-public-auth | 
[**update_game_namespace_matchmaker_config**](PortalApi.md#update_game_namespace_matchmaker_config) | **POST** /games/{game_id}/namespaces/{namespace_id}/mm-config | 
[**update_game_namespace_version**](PortalApi.md#update_game_namespace_version) | **PUT** /games/{game_id}/namespaces/{namespace_id}/version | 
[**update_namespace_domain**](PortalApi.md#update_namespace_domain) | **POST** /games/{game_id}/namespaces/{namespace_id}/domains/{domain} | 
[**validate_game**](PortalApi.md#validate_game) | **POST** /games/validate | 
[**validate_game_namespace**](PortalApi.md#validate_game_namespace) | **POST** /games/{game_id}/namespace/validate | 
[**validate_game_namespace_matchmaker_config**](PortalApi.md#validate_game_namespace_matchmaker_config) | **POST** /games/{game_id}/namespaces/{namespace_id}/mm-config/validate | 
[**validate_game_namespace_token_development**](PortalApi.md#validate_game_namespace_token_development) | **POST** /games/{game_id}/namespaces/{namespace_id}/tokens/development/validate | 
[**validate_game_version**](PortalApi.md#validate_game_version) | **POST** /games/{game_id}/version/validate | 
[**validate_team**](PortalApi.md#validate_team) | **POST** /teams/validate | 



## complete_upload

> serde_json::Value complete_upload(upload_id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**upload_id** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## convert_team

> convert_team(team_id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cloud_token

> crate::models::InlineResponse2007 create_cloud_token(game_id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::InlineResponse2007**](inline_response_200_7.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_game

> crate::models::InlineResponse200AgentGameCloud create_game(inline_object)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object** | [**InlineObject**](InlineObject.md) |  | [required] |

### Return type

[**crate::models::InlineResponse200AgentGameCloud**](inline_response_200_agent_game_cloud.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_game_build

> crate::models::InlineResponse20014 create_game_build(game_id, inline_object8)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |
**inline_object8** | [**InlineObject8**](InlineObject8.md) |  | [required] |

### Return type

[**crate::models::InlineResponse20014**](inline_response_200_14.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_game_cdn_site

> crate::models::InlineResponse20012 create_game_cdn_site(game_id, inline_object7)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |
**inline_object7** | [**InlineObject7**](InlineObject7.md) |  | [required] |

### Return type

[**crate::models::InlineResponse20012**](inline_response_200_12.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_game_namespace

> crate::models::InlineResponse2005 create_game_namespace(game_id, inline_object2)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |
**inline_object2** | [**InlineObject2**](InlineObject2.md) |  | [required] |

### Return type

[**crate::models::InlineResponse2005**](inline_response_200_5.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_game_namespace_token_development

> crate::models::InlineResponse2007 create_game_namespace_token_development(game_id, namespace_id, inline_object4)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |
**namespace_id** | **String** |  | [required] |
**inline_object4** | [**InlineObject4**](InlineObject4.md) |  | [required] |

### Return type

[**crate::models::InlineResponse2007**](inline_response_200_7.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_game_namespace_token_public

> crate::models::InlineResponse2007 create_game_namespace_token_public(game_id, namespace_id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |
**namespace_id** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::InlineResponse2007**](inline_response_200_7.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_game_version

> crate::models::InlineResponse2003 create_game_version(game_id, inline_object1)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |
**inline_object1** | [**InlineObject1**](InlineObject1.md) |  | [required] |

### Return type

[**crate::models::InlineResponse2003**](inline_response_200_3.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_matchmaker_lobby

> crate::models::InlineResponse20015 delete_matchmaker_lobby(game_id, lobby_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |
**lobby_id** | **String** |  | [required] |

### Return type

[**crate::models::InlineResponse20015**](inline_response_200_15.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_matchmaker_lobby_history

> crate::models::MatchmakerLobbiesExportHistoryResponse export_matchmaker_lobby_history(game_id, matchmaker_lobbies_export_history_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |
**matchmaker_lobbies_export_history_request** | [**MatchmakerLobbiesExportHistoryRequest**](MatchmakerLobbiesExportHistoryRequest.md) |  | [required] |

### Return type

[**crate::models::MatchmakerLobbiesExportHistoryResponse**](MatchmakerLobbiesExportHistoryResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_game_by_id

> crate::models::InlineResponse2002 get_game_by_id(game_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |

### Return type

[**crate::models::InlineResponse2002**](inline_response_200_2.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_game_namespace_by_id

> crate::models::InlineResponse2006 get_game_namespace_by_id(game_id, namespace_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |
**namespace_id** | **String** |  | [required] |

### Return type

[**crate::models::InlineResponse2006**](inline_response_200_6.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_game_version_by_id

> crate::models::InlineResponse2004 get_game_version_by_id(game_id, version_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |
**version_id** | **String** |  | [required] |

### Return type

[**crate::models::InlineResponse2004**](inline_response_200_4.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_games

> crate::models::InlineResponse2001 get_games()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse2001**](inline_response_200_1.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_namespace_analytics_matchmaker_live

> crate::models::InlineResponse2008 get_namespace_analytics_matchmaker_live(game_id, namespace_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |
**namespace_id** | **String** |  | [required] |

### Return type

[**crate::models::InlineResponse2008**](inline_response_200_8.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_namespace_lobby

> crate::models::InlineResponse20010 get_namespace_lobby(game_id, namespace_id, lobby_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |
**namespace_id** | **String** |  | [required] |
**lobby_id** | **String** |  | [required] |

### Return type

[**crate::models::InlineResponse20010**](inline_response_200_10.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ray_perf_logs

> crate::models::InlineResponse20021 get_ray_perf_logs(ray_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ray_id** | **String** |  | [required] |

### Return type

[**crate::models::InlineResponse20021**](inline_response_200_21.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_region_tiers

> crate::models::InlineResponse20020 get_region_tiers()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse20020**](inline_response_200_20.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_bank_transfers_list

> crate::models::InlineResponse20018 get_team_bank_transfers_list(team_id, start_transfer_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** |  | [required] |
**start_transfer_id** | Option<**String**> |  |  |

### Return type

[**crate::models::InlineResponse20018**](inline_response_200_18.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_billing

> crate::models::InlineResponse20016 get_team_billing(team_id, query_start, query_end)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** |  | [required] |
**query_start** | Option<**i64**> |  |  |
**query_end** | Option<**i64**> |  |  |

### Return type

[**crate::models::InlineResponse20016**](inline_response_200_16.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_payments_list

> crate::models::InlineResponse20017 get_team_payments_list(team_id, start_payment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** |  | [required] |
**start_payment_id** | Option<**String**> |  |  |

### Return type

[**crate::models::InlineResponse20017**](inline_response_200_17.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## inspect

> crate::models::InlineResponse200 inspect()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse200**](inline_response_200.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cdn_sites

> crate::models::InlineResponse20011 list_cdn_sites(game_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |

### Return type

[**crate::models::InlineResponse20011**](inline_response_200_11.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_game_builds

> crate::models::InlineResponse20013 list_game_builds(game_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |

### Return type

[**crate::models::InlineResponse20013**](inline_response_200_13.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_namespace_lobbies

> crate::models::InlineResponse2009 list_namespace_lobbies(game_id, namespace_id, before_create_ts)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |
**namespace_id** | **String** |  | [required] |
**before_create_ts** | Option<**i64**> |  |  |

### Return type

[**crate::models::InlineResponse2009**](inline_response_200_9.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_namespace_domain

> serde_json::Value remove_namespace_domain(game_id, namespace_id, domain)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |
**namespace_id** | **String** |  | [required] |
**domain** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_billing_checkout

> crate::models::InlineResponse20019 team_billing_checkout(team_id, inline_object9)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** |  | [required] |
**inline_object9** | [**InlineObject9**](InlineObject9.md) |  | [required] |

### Return type

[**crate::models::InlineResponse20019**](inline_response_200_19.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## toggle_namespace_domain_public_auth

> serde_json::Value toggle_namespace_domain_public_auth(game_id, namespace_id, inline_object5)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |
**namespace_id** | **String** |  | [required] |
**inline_object5** | [**InlineObject5**](InlineObject5.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_game_namespace_matchmaker_config

> serde_json::Value update_game_namespace_matchmaker_config(game_id, namespace_id, inline_object6)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |
**namespace_id** | **String** |  | [required] |
**inline_object6** | [**InlineObject6**](InlineObject6.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_game_namespace_version

> serde_json::Value update_game_namespace_version(game_id, namespace_id, inline_object3)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |
**namespace_id** | **String** |  | [required] |
**inline_object3** | [**InlineObject3**](InlineObject3.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_namespace_domain

> serde_json::Value update_namespace_domain(game_id, namespace_id, domain, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |
**namespace_id** | **String** |  | [required] |
**domain** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_game

> crate::models::ValidationErrors validate_game(inline_object12)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object12** | [**InlineObject12**](InlineObject12.md) |  | [required] |

### Return type

[**crate::models::ValidationErrors**](ValidationErrors.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_game_namespace

> crate::models::ValidationErrors validate_game_namespace(game_id, inline_object11)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |
**inline_object11** | [**InlineObject11**](InlineObject11.md) |  | [required] |

### Return type

[**crate::models::ValidationErrors**](ValidationErrors.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_game_namespace_matchmaker_config

> crate::models::ValidationErrors validate_game_namespace_matchmaker_config(game_id, namespace_id, inline_object15)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |
**namespace_id** | **String** |  | [required] |
**inline_object15** | [**InlineObject15**](InlineObject15.md) |  | [required] |

### Return type

[**crate::models::ValidationErrors**](ValidationErrors.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_game_namespace_token_development

> crate::models::ValidationErrors validate_game_namespace_token_development(game_id, namespace_id, inline_object14)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |
**namespace_id** | **String** |  | [required] |
**inline_object14** | [**InlineObject14**](InlineObject14.md) |  | [required] |

### Return type

[**crate::models::ValidationErrors**](ValidationErrors.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_game_version

> crate::models::ValidationErrors validate_game_version(game_id, inline_object10)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |
**inline_object10** | [**InlineObject10**](InlineObject10.md) |  | [required] |

### Return type

[**crate::models::ValidationErrors**](ValidationErrors.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_team

> crate::models::ValidationErrors validate_team(inline_object13)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object13** | [**InlineObject13**](InlineObject13.md) |  | [required] |

### Return type

[**crate::models::ValidationErrors**](ValidationErrors.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

