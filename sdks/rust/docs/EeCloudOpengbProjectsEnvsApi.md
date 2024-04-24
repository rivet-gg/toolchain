# \EeCloudOpengbProjectsEnvsApi

All URIs are relative to *https://api.rivet.gg*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ee_cloud_opengb_projects_envs_create**](EeCloudOpengbProjectsEnvsApi.md#ee_cloud_opengb_projects_envs_create) | **POST** /cloud/opengb/projects/{project_id}/environments | 
[**ee_cloud_opengb_projects_envs_delete**](EeCloudOpengbProjectsEnvsApi.md#ee_cloud_opengb_projects_envs_delete) | **DELETE** /cloud/opengb/projects/{project_id}/environments/{environment_id} | 
[**ee_cloud_opengb_projects_envs_delete_secrets**](EeCloudOpengbProjectsEnvsApi.md#ee_cloud_opengb_projects_envs_delete_secrets) | **DELETE** /cloud/opengb/projects/{project_id}/environments/{environment_id}/secrets | 
[**ee_cloud_opengb_projects_envs_deploy**](EeCloudOpengbProjectsEnvsApi.md#ee_cloud_opengb_projects_envs_deploy) | **POST** /cloud/opengb/projects/{project_id}/environments/{environment_id}/deploy | 
[**ee_cloud_opengb_projects_envs_get**](EeCloudOpengbProjectsEnvsApi.md#ee_cloud_opengb_projects_envs_get) | **GET** /cloud/opengb/projects/{project_id}/environments/{environment_id} | 
[**ee_cloud_opengb_projects_envs_get_db_urls**](EeCloudOpengbProjectsEnvsApi.md#ee_cloud_opengb_projects_envs_get_db_urls) | **GET** /cloud/opengb/projects/{project_id}/environments/{environment_id}/db | 
[**ee_cloud_opengb_projects_envs_list**](EeCloudOpengbProjectsEnvsApi.md#ee_cloud_opengb_projects_envs_list) | **GET** /cloud/opengb/projects/{project_id}/environments | 
[**ee_cloud_opengb_projects_envs_prepare_deploy**](EeCloudOpengbProjectsEnvsApi.md#ee_cloud_opengb_projects_envs_prepare_deploy) | **POST** /cloud/opengb/projects/{project_id}/environments/{environment_id}/deploy/prepare | 
[**ee_cloud_opengb_projects_envs_provision_databases**](EeCloudOpengbProjectsEnvsApi.md#ee_cloud_opengb_projects_envs_provision_databases) | **POST** /cloud/opengb/projects/{project_id}/environments/{environment_id}/provision-databases | 
[**ee_cloud_opengb_projects_envs_update_config**](EeCloudOpengbProjectsEnvsApi.md#ee_cloud_opengb_projects_envs_update_config) | **POST** /cloud/opengb/projects/{project_id}/environments/{environment_id}/config | 
[**ee_cloud_opengb_projects_envs_update_secrets**](EeCloudOpengbProjectsEnvsApi.md#ee_cloud_opengb_projects_envs_update_secrets) | **POST** /cloud/opengb/projects/{project_id}/environments/{environment_id}/secrets | 
[**ee_cloud_opengb_projects_envs_validate**](EeCloudOpengbProjectsEnvsApi.md#ee_cloud_opengb_projects_envs_validate) | **POST** /cloud/opengb/projects/{project_id}/environments/validate | 



## ee_cloud_opengb_projects_envs_create

> crate::models::EeCloudOpengbProjectsEnvsCreateResponse ee_cloud_opengb_projects_envs_create(project_id, ee_cloud_opengb_projects_envs_create_request)


Creates a new OpenGB environment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** |  | [required] |
**ee_cloud_opengb_projects_envs_create_request** | [**EeCloudOpengbProjectsEnvsCreateRequest**](EeCloudOpengbProjectsEnvsCreateRequest.md) |  | [required] |

### Return type

[**crate::models::EeCloudOpengbProjectsEnvsCreateResponse**](EeCloudOpengbProjectsEnvsCreateResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_cloud_opengb_projects_envs_delete

> ee_cloud_opengb_projects_envs_delete(project_id, environment_id)


Deletes an OpenGB environment.

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


## ee_cloud_opengb_projects_envs_delete_secrets

> ee_cloud_opengb_projects_envs_delete_secrets(project_id, environment_id, ee_cloud_opengb_projects_envs_delete_secrets_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** |  | [required] |
**environment_id** | **uuid::Uuid** |  | [required] |
**ee_cloud_opengb_projects_envs_delete_secrets_request** | [**EeCloudOpengbProjectsEnvsDeleteSecretsRequest**](EeCloudOpengbProjectsEnvsDeleteSecretsRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_cloud_opengb_projects_envs_deploy

> crate::models::EeCloudOpengbProjectsEnvsDeployResponse ee_cloud_opengb_projects_envs_deploy(project_id, environment_id, ee_cloud_opengb_projects_envs_deploy_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** |  | [required] |
**environment_id** | **uuid::Uuid** |  | [required] |
**ee_cloud_opengb_projects_envs_deploy_request** | [**EeCloudOpengbProjectsEnvsDeployRequest**](EeCloudOpengbProjectsEnvsDeployRequest.md) |  | [required] |

### Return type

[**crate::models::EeCloudOpengbProjectsEnvsDeployResponse**](EeCloudOpengbProjectsEnvsDeployResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_cloud_opengb_projects_envs_get

> crate::models::EeCloudOpengbProjectsEnvsGetResponse ee_cloud_opengb_projects_envs_get(project_id, environment_id, watch_index)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** |  | [required] |
**environment_id** | **uuid::Uuid** |  | [required] |
**watch_index** | Option<**String**> | A query parameter denoting the requests watch index. |  |

### Return type

[**crate::models::EeCloudOpengbProjectsEnvsGetResponse**](EeCloudOpengbProjectsEnvsGetResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_cloud_opengb_projects_envs_get_db_urls

> crate::models::EeCloudOpengbProjectsEnvsGetDbUrlsResponse ee_cloud_opengb_projects_envs_get_db_urls(project_id, environment_id, dbs)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** |  | [required] |
**environment_id** | **uuid::Uuid** |  | [required] |
**dbs** | **String** | Module database names. | [required] |

### Return type

[**crate::models::EeCloudOpengbProjectsEnvsGetDbUrlsResponse**](EeCloudOpengbProjectsEnvsGetDbUrlsResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_cloud_opengb_projects_envs_list

> crate::models::EeCloudOpengbProjectsEnvsListResponse ee_cloud_opengb_projects_envs_list(project_id, watch_index)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** |  | [required] |
**watch_index** | Option<**String**> | A query parameter denoting the requests watch index. |  |

### Return type

[**crate::models::EeCloudOpengbProjectsEnvsListResponse**](EeCloudOpengbProjectsEnvsListResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_cloud_opengb_projects_envs_prepare_deploy

> crate::models::EeCloudOpengbProjectsEnvsPrepareDeployResponse ee_cloud_opengb_projects_envs_prepare_deploy(project_id, environment_id, ee_cloud_opengb_projects_envs_prepare_deploy_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** |  | [required] |
**environment_id** | **uuid::Uuid** |  | [required] |
**ee_cloud_opengb_projects_envs_prepare_deploy_request** | [**EeCloudOpengbProjectsEnvsPrepareDeployRequest**](EeCloudOpengbProjectsEnvsPrepareDeployRequest.md) |  | [required] |

### Return type

[**crate::models::EeCloudOpengbProjectsEnvsPrepareDeployResponse**](EeCloudOpengbProjectsEnvsPrepareDeployResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_cloud_opengb_projects_envs_provision_databases

> ee_cloud_opengb_projects_envs_provision_databases(project_id, environment_id, ee_cloud_opengb_projects_envs_provision_databases_request)


Provision all of the databases for the given modules. Idempotent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** |  | [required] |
**environment_id** | **uuid::Uuid** |  | [required] |
**ee_cloud_opengb_projects_envs_provision_databases_request** | [**EeCloudOpengbProjectsEnvsProvisionDatabasesRequest**](EeCloudOpengbProjectsEnvsProvisionDatabasesRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_cloud_opengb_projects_envs_update_config

> ee_cloud_opengb_projects_envs_update_config(project_id, environment_id, ee_cloud_opengb_projects_envs_update_config_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** |  | [required] |
**environment_id** | **uuid::Uuid** |  | [required] |
**ee_cloud_opengb_projects_envs_update_config_request** | [**EeCloudOpengbProjectsEnvsUpdateConfigRequest**](EeCloudOpengbProjectsEnvsUpdateConfigRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_cloud_opengb_projects_envs_update_secrets

> ee_cloud_opengb_projects_envs_update_secrets(project_id, environment_id, ee_cloud_opengb_projects_envs_update_secrets_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** |  | [required] |
**environment_id** | **uuid::Uuid** |  | [required] |
**ee_cloud_opengb_projects_envs_update_secrets_request** | [**EeCloudOpengbProjectsEnvsUpdateSecretsRequest**](EeCloudOpengbProjectsEnvsUpdateSecretsRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_cloud_opengb_projects_envs_validate

> crate::models::EeCloudOpengbProjectsEnvsValidateResponse ee_cloud_opengb_projects_envs_validate(project_id, ee_cloud_opengb_projects_envs_validate_request)


Validates information used to create a new environment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** |  | [required] |
**ee_cloud_opengb_projects_envs_validate_request** | [**EeCloudOpengbProjectsEnvsValidateRequest**](EeCloudOpengbProjectsEnvsValidateRequest.md) |  | [required] |

### Return type

[**crate::models::EeCloudOpengbProjectsEnvsValidateResponse**](EeCloudOpengbProjectsEnvsValidateResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

