# \CloudGamesNamespacesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**namespaces_add_namespace_domain**](CloudGamesNamespacesApi.md#namespaces_add_namespace_domain) | **POST** /games/{game_id}/namespaces/{namespace_id}/domains | 
[**namespaces_create_game_namespace**](CloudGamesNamespacesApi.md#namespaces_create_game_namespace) | **POST** /games/{game_id}/namespaces | 
[**namespaces_create_game_namespace_token_development**](CloudGamesNamespacesApi.md#namespaces_create_game_namespace_token_development) | **POST** /games/{game_id}/namespaces/{namespace_id}/tokens/development | 
[**namespaces_create_game_namespace_token_public**](CloudGamesNamespacesApi.md#namespaces_create_game_namespace_token_public) | **POST** /games/{game_id}/namespaces/{namespace_id}/tokens/public | 
[**namespaces_get_game_namespace_by_id**](CloudGamesNamespacesApi.md#namespaces_get_game_namespace_by_id) | **GET** /games/{game_id}/namespaces/{namespace_id} | 
[**namespaces_remove_namespace_cdn_auth_user**](CloudGamesNamespacesApi.md#namespaces_remove_namespace_cdn_auth_user) | **DELETE** /games/{game_id}/namespaces/{namespace_id}/auth-user/{user} | 
[**namespaces_remove_namespace_domain**](CloudGamesNamespacesApi.md#namespaces_remove_namespace_domain) | **DELETE** /games/{game_id}/namespaces/{namespace_id}/domains/{domain} | 
[**namespaces_set_namespace_cdn_auth_type**](CloudGamesNamespacesApi.md#namespaces_set_namespace_cdn_auth_type) | **PUT** /games/{game_id}/namespaces/{namespace_id}/cdn-auth | 
[**namespaces_toggle_namespace_domain_public_auth**](CloudGamesNamespacesApi.md#namespaces_toggle_namespace_domain_public_auth) | **PUT** /games/{game_id}/namespaces/{namespace_id}/domain-public-auth | 
[**namespaces_update_game_namespace_matchmaker_config**](CloudGamesNamespacesApi.md#namespaces_update_game_namespace_matchmaker_config) | **POST** /games/{game_id}/namespaces/{namespace_id}/mm-config | 
[**namespaces_update_game_namespace_version**](CloudGamesNamespacesApi.md#namespaces_update_game_namespace_version) | **PUT** /games/{game_id}/namespaces/{namespace_id}/version | 
[**namespaces_update_namespace_cdn_auth_user**](CloudGamesNamespacesApi.md#namespaces_update_namespace_cdn_auth_user) | **POST** /games/{game_id}/namespaces/{namespace_id}/auth-user | 
[**namespaces_validate_game_namespace**](CloudGamesNamespacesApi.md#namespaces_validate_game_namespace) | **POST** /games/{game_id}/namespaces/validate | 
[**namespaces_validate_game_namespace_matchmaker_config**](CloudGamesNamespacesApi.md#namespaces_validate_game_namespace_matchmaker_config) | **POST** /games/{game_id}/namespaces/{namespace_id}/mm-config/validate | 
[**namespaces_validate_game_namespace_token_development**](CloudGamesNamespacesApi.md#namespaces_validate_game_namespace_token_development) | **POST** /games/{game_id}/namespaces/{namespace_id}/tokens/development/validate | 



## namespaces_add_namespace_domain

> namespaces_add_namespace_domain(game_id, namespace_id, cloud_games_add_namespace_domain_input)


Adds a domain to the given game namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | A universally unique identifier. | [required] |
**namespace_id** | **String** | A universally unique identifier. | [required] |
**cloud_games_add_namespace_domain_input** | [**CloudGamesAddNamespaceDomainInput**](CloudGamesAddNamespaceDomainInput.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_create_game_namespace

> crate::models::CloudGamesCreateGameNamespaceOutput namespaces_create_game_namespace(game_id, cloud_games_create_game_namespace_input)


Creates a new namespace for the given game.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | A universally unique identifier. | [required] |
**cloud_games_create_game_namespace_input** | [**CloudGamesCreateGameNamespaceInput**](CloudGamesCreateGameNamespaceInput.md) |  | [required] |

### Return type

[**crate::models::CloudGamesCreateGameNamespaceOutput**](CloudGamesCreateGameNamespaceOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_create_game_namespace_token_development

> crate::models::CloudGamesCreateGameNamespaceTokenDevelopmentOutput namespaces_create_game_namespace_token_development(game_id, namespace_id, cloud_games_create_game_namespace_token_development_input)


Creates a development token for the given namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | A universally unique identifier. | [required] |
**namespace_id** | **String** | A universally unique identifier. | [required] |
**cloud_games_create_game_namespace_token_development_input** | [**CloudGamesCreateGameNamespaceTokenDevelopmentInput**](CloudGamesCreateGameNamespaceTokenDevelopmentInput.md) |  | [required] |

### Return type

[**crate::models::CloudGamesCreateGameNamespaceTokenDevelopmentOutput**](CloudGamesCreateGameNamespaceTokenDevelopmentOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_create_game_namespace_token_public

> crate::models::CloudGamesCreateGameNamespaceTokenPublicOutput namespaces_create_game_namespace_token_public(game_id, namespace_id)


Creates a public token for the given namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | A universally unique identifier. | [required] |
**namespace_id** | **String** | A universally unique identifier. | [required] |

### Return type

[**crate::models::CloudGamesCreateGameNamespaceTokenPublicOutput**](CloudGamesCreateGameNamespaceTokenPublicOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_game_namespace_by_id

> crate::models::CloudGamesGetGameNamespaceByIdOutput namespaces_get_game_namespace_by_id(game_id, namespace_id)


Gets a game namespace by namespace ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | A universally unique identifier. | [required] |
**namespace_id** | **String** | A universally unique identifier. | [required] |

### Return type

[**crate::models::CloudGamesGetGameNamespaceByIdOutput**](CloudGamesGetGameNamespaceByIdOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_remove_namespace_cdn_auth_user

> namespaces_remove_namespace_cdn_auth_user(game_id, namespace_id, user)


Removes an authenticated user from the given game namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | A universally unique identifier. | [required] |
**namespace_id** | **String** | A universally unique identifier. | [required] |
**user** | **String** | A user name. | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_remove_namespace_domain

> namespaces_remove_namespace_domain(game_id, namespace_id, domain)


Removes a domain from the given game namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | A universally unique identifier. | [required] |
**namespace_id** | **String** | A universally unique identifier. | [required] |
**domain** | **String** | A valid domain name (no protocol). | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_namespace_cdn_auth_type

> namespaces_set_namespace_cdn_auth_type(game_id, namespace_id, cloud_games_set_namespace_cdn_auth_type_input)


Updates the CDN authentication type of the given game namesapce.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | A universally unique identifier. | [required] |
**namespace_id** | **String** | A universally unique identifier. | [required] |
**cloud_games_set_namespace_cdn_auth_type_input** | [**CloudGamesSetNamespaceCdnAuthTypeInput**](CloudGamesSetNamespaceCdnAuthTypeInput.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_toggle_namespace_domain_public_auth

> namespaces_toggle_namespace_domain_public_auth(game_id, namespace_id, cloud_games_toggle_namespace_domain_public_auth_input)


Toggles whether or not to allow authentication based on domain for the given game namesapce.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | A universally unique identifier. | [required] |
**namespace_id** | **String** | A universally unique identifier. | [required] |
**cloud_games_toggle_namespace_domain_public_auth_input** | [**CloudGamesToggleNamespaceDomainPublicAuthInput**](CloudGamesToggleNamespaceDomainPublicAuthInput.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_update_game_namespace_matchmaker_config

> namespaces_update_game_namespace_matchmaker_config(game_id, namespace_id, cloud_games_update_game_namespace_matchmaker_config_input)


Updates matchmaker config for the given game namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | A universally unique identifier. | [required] |
**namespace_id** | **String** | A universally unique identifier. | [required] |
**cloud_games_update_game_namespace_matchmaker_config_input** | [**CloudGamesUpdateGameNamespaceMatchmakerConfigInput**](CloudGamesUpdateGameNamespaceMatchmakerConfigInput.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_update_game_namespace_version

> namespaces_update_game_namespace_version(game_id, namespace_id, cloud_games_update_game_namespace_version_input)


Updates the version of a game namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | A universally unique identifier. | [required] |
**namespace_id** | **String** | A universally unique identifier. | [required] |
**cloud_games_update_game_namespace_version_input** | [**CloudGamesUpdateGameNamespaceVersionInput**](CloudGamesUpdateGameNamespaceVersionInput.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_update_namespace_cdn_auth_user

> namespaces_update_namespace_cdn_auth_user(game_id, namespace_id, cloud_games_update_namespace_cdn_auth_user_input)


Adds an authenticated user to the given game namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | A universally unique identifier. | [required] |
**namespace_id** | **String** | A universally unique identifier. | [required] |
**cloud_games_update_namespace_cdn_auth_user_input** | [**CloudGamesUpdateNamespaceCdnAuthUserInput**](CloudGamesUpdateNamespaceCdnAuthUserInput.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_validate_game_namespace

> crate::models::CloudGamesValidateGameNamespaceOutput namespaces_validate_game_namespace(game_id, cloud_games_validate_game_namespace_input)


Validates information used to create a new game namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | A universally unique identifier. | [required] |
**cloud_games_validate_game_namespace_input** | [**CloudGamesValidateGameNamespaceInput**](CloudGamesValidateGameNamespaceInput.md) |  | [required] |

### Return type

[**crate::models::CloudGamesValidateGameNamespaceOutput**](CloudGamesValidateGameNamespaceOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_validate_game_namespace_matchmaker_config

> crate::models::CloudGamesValidateGameNamespaceMatchmakerConfigOutput namespaces_validate_game_namespace_matchmaker_config(game_id, namespace_id, cloud_games_validate_game_namespace_matchmaker_config_input)


Validates information used to update a game namespace's matchmaker config.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | A universally unique identifier. | [required] |
**namespace_id** | **String** | A universally unique identifier. | [required] |
**cloud_games_validate_game_namespace_matchmaker_config_input** | [**CloudGamesValidateGameNamespaceMatchmakerConfigInput**](CloudGamesValidateGameNamespaceMatchmakerConfigInput.md) |  | [required] |

### Return type

[**crate::models::CloudGamesValidateGameNamespaceMatchmakerConfigOutput**](CloudGamesValidateGameNamespaceMatchmakerConfigOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_validate_game_namespace_token_development

> crate::models::CloudGamesValidateGameNamespaceTokenDevelopmentOutput namespaces_validate_game_namespace_token_development(game_id, namespace_id, cloud_games_validate_game_namespace_token_development_input)


Validates information used to create a new game namespace development token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | A universally unique identifier. | [required] |
**namespace_id** | **String** | A universally unique identifier. | [required] |
**cloud_games_validate_game_namespace_token_development_input** | [**CloudGamesValidateGameNamespaceTokenDevelopmentInput**](CloudGamesValidateGameNamespaceTokenDevelopmentInput.md) |  | [required] |

### Return type

[**crate::models::CloudGamesValidateGameNamespaceTokenDevelopmentOutput**](CloudGamesValidateGameNamespaceTokenDevelopmentOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

