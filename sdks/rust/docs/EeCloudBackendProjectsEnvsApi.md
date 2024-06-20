# \EeCloudBackendProjectsEnvsApi

All URIs are relative to *https://api.rivet.gg*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ee_cloud_backend_projects_envs_create**](EeCloudBackendProjectsEnvsApi.md#ee_cloud_backend_projects_envs_create) | **POST** /cloud/backend/projects/{project_id}/environments | 
[**ee_cloud_backend_projects_envs_delete**](EeCloudBackendProjectsEnvsApi.md#ee_cloud_backend_projects_envs_delete) | **DELETE** /cloud/backend/projects/{project_id}/environments/{environment_id} | 
[**ee_cloud_backend_projects_envs_deploy**](EeCloudBackendProjectsEnvsApi.md#ee_cloud_backend_projects_envs_deploy) | **POST** /cloud/backend/projects/{project_id}/environments/{environment_id}/deploy | 
[**ee_cloud_backend_projects_envs_get**](EeCloudBackendProjectsEnvsApi.md#ee_cloud_backend_projects_envs_get) | **GET** /cloud/backend/projects/{project_id}/environments/{environment_id} | 
[**ee_cloud_backend_projects_envs_get_config**](EeCloudBackendProjectsEnvsApi.md#ee_cloud_backend_projects_envs_get_config) | **GET** /cloud/backend/projects/{project_id}/environments/{environment_id}/config | 
[**ee_cloud_backend_projects_envs_get_db_url**](EeCloudBackendProjectsEnvsApi.md#ee_cloud_backend_projects_envs_get_db_url) | **GET** /cloud/backend/projects/{project_id}/environments/{environment_id}/db | 
[**ee_cloud_backend_projects_envs_get_events**](EeCloudBackendProjectsEnvsApi.md#ee_cloud_backend_projects_envs_get_events) | **GET** /cloud/backend/projects/{project_id}/environments/{environment_id}/events | 
[**ee_cloud_backend_projects_envs_get_variables**](EeCloudBackendProjectsEnvsApi.md#ee_cloud_backend_projects_envs_get_variables) | **GET** /cloud/backend/projects/{project_id}/environments/{environment_id}/variables | 
[**ee_cloud_backend_projects_envs_list**](EeCloudBackendProjectsEnvsApi.md#ee_cloud_backend_projects_envs_list) | **GET** /cloud/backend/projects/{project_id}/environments | 
[**ee_cloud_backend_projects_envs_prepare_deploy**](EeCloudBackendProjectsEnvsApi.md#ee_cloud_backend_projects_envs_prepare_deploy) | **POST** /cloud/backend/projects/{project_id}/environments/{environment_id}/deploy/prepare | 
[**ee_cloud_backend_projects_envs_provision_database**](EeCloudBackendProjectsEnvsApi.md#ee_cloud_backend_projects_envs_provision_database) | **POST** /cloud/backend/projects/{project_id}/environments/{environment_id}/provision-database | 
[**ee_cloud_backend_projects_envs_update_config**](EeCloudBackendProjectsEnvsApi.md#ee_cloud_backend_projects_envs_update_config) | **POST** /cloud/backend/projects/{project_id}/environments/{environment_id}/config | 
[**ee_cloud_backend_projects_envs_update_variables**](EeCloudBackendProjectsEnvsApi.md#ee_cloud_backend_projects_envs_update_variables) | **PATCH** /cloud/backend/projects/{project_id}/environments/{environment_id}/variables | 
[**ee_cloud_backend_projects_envs_validate**](EeCloudBackendProjectsEnvsApi.md#ee_cloud_backend_projects_envs_validate) | **POST** /cloud/backend/projects/{project_id}/environments/validate | 



## ee_cloud_backend_projects_envs_create

> crate::models::EeCloudBackendProjectsEnvsCreateResponse ee_cloud_backend_projects_envs_create(project_id, ee_cloud_backend_projects_envs_create_request)


Creates a new OpenGB environment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** |  | [required] |
**ee_cloud_backend_projects_envs_create_request** | [**EeCloudBackendProjectsEnvsCreateRequest**](EeCloudBackendProjectsEnvsCreateRequest.md) |  | [required] |

### Return type

[**crate::models::EeCloudBackendProjectsEnvsCreateResponse**](EeCloudBackendProjectsEnvsCreateResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_cloud_backend_projects_envs_delete

> ee_cloud_backend_projects_envs_delete(project_id, environment_id)


Deletes an backend environment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** |  | [required] |
**environment_id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_cloud_backend_projects_envs_deploy

> crate::models::EeCloudBackendProjectsEnvsDeployResponse ee_cloud_backend_projects_envs_deploy(project_id, environment_id, ee_cloud_backend_projects_envs_deploy_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** |  | [required] |
**environment_id** | **uuid::Uuid** |  | [required] |
**ee_cloud_backend_projects_envs_deploy_request** | [**EeCloudBackendProjectsEnvsDeployRequest**](EeCloudBackendProjectsEnvsDeployRequest.md) |  | [required] |

### Return type

[**crate::models::EeCloudBackendProjectsEnvsDeployResponse**](EeCloudBackendProjectsEnvsDeployResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_cloud_backend_projects_envs_get

> crate::models::EeCloudBackendProjectsEnvsGetResponse ee_cloud_backend_projects_envs_get(project_id, environment_id, watch_index)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** |  | [required] |
**environment_id** | **uuid::Uuid** |  | [required] |
**watch_index** | Option<**String**> | A query parameter denoting the requests watch index. |  |

### Return type

[**crate::models::EeCloudBackendProjectsEnvsGetResponse**](EeCloudBackendProjectsEnvsGetResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_cloud_backend_projects_envs_get_config

> crate::models::EeCloudBackendProjectsEnvsGetConfigResponse ee_cloud_backend_projects_envs_get_config(project_id, environment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** |  | [required] |
**environment_id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::EeCloudBackendProjectsEnvsGetConfigResponse**](EeCloudBackendProjectsEnvsGetConfigResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_cloud_backend_projects_envs_get_db_url

> crate::models::EeCloudBackendProjectsEnvsGetDbUrlResponse ee_cloud_backend_projects_envs_get_db_url(project_id, environment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** |  | [required] |
**environment_id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::EeCloudBackendProjectsEnvsGetDbUrlResponse**](EeCloudBackendProjectsEnvsGetDbUrlResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_cloud_backend_projects_envs_get_events

> crate::models::EeCloudBackendProjectsEnvsGetEventsResponse ee_cloud_backend_projects_envs_get_events(project_id, environment_id, watch_index)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** |  | [required] |
**environment_id** | **uuid::Uuid** |  | [required] |
**watch_index** | Option<**String**> | A query parameter denoting the requests watch index. |  |

### Return type

[**crate::models::EeCloudBackendProjectsEnvsGetEventsResponse**](EeCloudBackendProjectsEnvsGetEventsResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_cloud_backend_projects_envs_get_variables

> crate::models::EeCloudBackendProjectsEnvsGetVariablesResponse ee_cloud_backend_projects_envs_get_variables(project_id, environment_id)


Get environment variables from an environment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** |  | [required] |
**environment_id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::EeCloudBackendProjectsEnvsGetVariablesResponse**](EeCloudBackendProjectsEnvsGetVariablesResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_cloud_backend_projects_envs_list

> crate::models::EeCloudBackendProjectsEnvsListResponse ee_cloud_backend_projects_envs_list(project_id, watch_index)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** |  | [required] |
**watch_index** | Option<**String**> | A query parameter denoting the requests watch index. |  |

### Return type

[**crate::models::EeCloudBackendProjectsEnvsListResponse**](EeCloudBackendProjectsEnvsListResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_cloud_backend_projects_envs_prepare_deploy

> crate::models::EeCloudBackendProjectsEnvsPrepareDeployResponse ee_cloud_backend_projects_envs_prepare_deploy(project_id, environment_id, ee_cloud_backend_projects_envs_prepare_deploy_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** |  | [required] |
**environment_id** | **uuid::Uuid** |  | [required] |
**ee_cloud_backend_projects_envs_prepare_deploy_request** | [**EeCloudBackendProjectsEnvsPrepareDeployRequest**](EeCloudBackendProjectsEnvsPrepareDeployRequest.md) |  | [required] |

### Return type

[**crate::models::EeCloudBackendProjectsEnvsPrepareDeployResponse**](EeCloudBackendProjectsEnvsPrepareDeployResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_cloud_backend_projects_envs_provision_database

> ee_cloud_backend_projects_envs_provision_database(project_id, environment_id)


Provisions the database for the given environment. Idempotent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** |  | [required] |
**environment_id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_cloud_backend_projects_envs_update_config

> ee_cloud_backend_projects_envs_update_config(project_id, environment_id, ee_cloud_backend_projects_envs_update_config_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** |  | [required] |
**environment_id** | **uuid::Uuid** |  | [required] |
**ee_cloud_backend_projects_envs_update_config_request** | [**EeCloudBackendProjectsEnvsUpdateConfigRequest**](EeCloudBackendProjectsEnvsUpdateConfigRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_cloud_backend_projects_envs_update_variables

> ee_cloud_backend_projects_envs_update_variables(project_id, environment_id, ee_cloud_backend_projects_envs_update_variables_request)


Updates environment variables for an environment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** |  | [required] |
**environment_id** | **uuid::Uuid** |  | [required] |
**ee_cloud_backend_projects_envs_update_variables_request** | [**EeCloudBackendProjectsEnvsUpdateVariablesRequest**](EeCloudBackendProjectsEnvsUpdateVariablesRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_cloud_backend_projects_envs_validate

> crate::models::EeCloudBackendProjectsEnvsValidateResponse ee_cloud_backend_projects_envs_validate(project_id, ee_cloud_backend_projects_envs_validate_request)


Validates information used to create a new environment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** |  | [required] |
**ee_cloud_backend_projects_envs_validate_request** | [**EeCloudBackendProjectsEnvsValidateRequest**](EeCloudBackendProjectsEnvsValidateRequest.md) |  | [required] |

### Return type

[**crate::models::EeCloudBackendProjectsEnvsValidateResponse**](EeCloudBackendProjectsEnvsValidateResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

