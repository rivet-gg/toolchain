# \EeBackendApi

All URIs are relative to *https://api.rivet.gg*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ee_backend_create**](EeBackendApi.md#ee_backend_create) | **POST** /games/{game_id}/environments/{environment_id}/backend | 
[**ee_backend_deploy**](EeBackendApi.md#ee_backend_deploy) | **POST** /games/{game_id}/environments/{environment_id}/backend/deploy | 
[**ee_backend_get**](EeBackendApi.md#ee_backend_get) | **GET** /games/{game_id}/environments/{environment_id}/backend | 
[**ee_backend_get_db_url**](EeBackendApi.md#ee_backend_get_db_url) | **GET** /games/{game_id}/environments/{environment_id}/backend/db | 
[**ee_backend_get_events**](EeBackendApi.md#ee_backend_get_events) | **GET** /games/{game_id}/environments/{environment_id}/backend/events | 
[**ee_backend_get_variables**](EeBackendApi.md#ee_backend_get_variables) | **GET** /games/{game_id}/environments/{environment_id}/backend/variables | 
[**ee_backend_prepare_deploy**](EeBackendApi.md#ee_backend_prepare_deploy) | **POST** /games/{game_id}/environments/{environment_id}/backend/deploy/prepare | 
[**ee_backend_provision_database**](EeBackendApi.md#ee_backend_provision_database) | **POST** /games/{game_id}/environments/{environment_id}/backend/provision-database | 
[**ee_backend_update_variables**](EeBackendApi.md#ee_backend_update_variables) | **PATCH** /games/{game_id}/environments/{environment_id}/backend/variables | 



## ee_backend_create

> crate::models::EeBackendCreateResponse ee_backend_create(game_id, environment_id, body)


Creates a new backend environment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **uuid::Uuid** |  | [required] |
**environment_id** | **uuid::Uuid** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::EeBackendCreateResponse**](EeBackendCreateResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_backend_deploy

> crate::models::EeBackendDeployResponse ee_backend_deploy(game_id, environment_id, ee_backend_deploy_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **uuid::Uuid** |  | [required] |
**environment_id** | **uuid::Uuid** |  | [required] |
**ee_backend_deploy_request** | [**EeBackendDeployRequest**](EeBackendDeployRequest.md) |  | [required] |

### Return type

[**crate::models::EeBackendDeployResponse**](EeBackendDeployResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_backend_get

> crate::models::EeBackendGetResponse ee_backend_get(game_id, environment_id, watch_index)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **uuid::Uuid** |  | [required] |
**environment_id** | **uuid::Uuid** |  | [required] |
**watch_index** | Option<**String**> | A query parameter denoting the requests watch index. |  |

### Return type

[**crate::models::EeBackendGetResponse**](EeBackendGetResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_backend_get_db_url

> crate::models::EeBackendGetDbUrlResponse ee_backend_get_db_url(game_id, environment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **uuid::Uuid** |  | [required] |
**environment_id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::EeBackendGetDbUrlResponse**](EeBackendGetDbUrlResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_backend_get_events

> crate::models::EeBackendGetEventsResponse ee_backend_get_events(game_id, environment_id, watch_index)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **uuid::Uuid** |  | [required] |
**environment_id** | **uuid::Uuid** |  | [required] |
**watch_index** | Option<**String**> | A query parameter denoting the requests watch index. |  |

### Return type

[**crate::models::EeBackendGetEventsResponse**](EeBackendGetEventsResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_backend_get_variables

> crate::models::EeBackendGetVariablesResponse ee_backend_get_variables(game_id, environment_id)


Get backend variables from an backend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **uuid::Uuid** |  | [required] |
**environment_id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::EeBackendGetVariablesResponse**](EeBackendGetVariablesResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_backend_prepare_deploy

> crate::models::EeBackendPrepareDeployResponse ee_backend_prepare_deploy(game_id, environment_id, ee_backend_prepare_deploy_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **uuid::Uuid** |  | [required] |
**environment_id** | **uuid::Uuid** |  | [required] |
**ee_backend_prepare_deploy_request** | [**EeBackendPrepareDeployRequest**](EeBackendPrepareDeployRequest.md) |  | [required] |

### Return type

[**crate::models::EeBackendPrepareDeployResponse**](EeBackendPrepareDeployResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_backend_provision_database

> ee_backend_provision_database(game_id, environment_id)


Provisions the database for the given backend. Idempotent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **uuid::Uuid** |  | [required] |
**environment_id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_backend_update_variables

> ee_backend_update_variables(game_id, environment_id, ee_backend_update_variables_request)


Updates backend variables for an backend.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **uuid::Uuid** |  | [required] |
**environment_id** | **uuid::Uuid** |  | [required] |
**ee_backend_update_variables_request** | [**EeBackendUpdateVariablesRequest**](EeBackendUpdateVariablesRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

