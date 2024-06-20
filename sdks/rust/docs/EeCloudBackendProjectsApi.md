# \EeCloudBackendProjectsApi

All URIs are relative to *https://api.rivet.gg*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ee_cloud_backend_projects_create**](EeCloudBackendProjectsApi.md#ee_cloud_backend_projects_create) | **POST** /cloud/backend/projects | 
[**ee_cloud_backend_projects_get**](EeCloudBackendProjectsApi.md#ee_cloud_backend_projects_get) | **GET** /cloud/backend/projects/{project_id} | 
[**ee_cloud_backend_projects_list**](EeCloudBackendProjectsApi.md#ee_cloud_backend_projects_list) | **GET** /cloud/backend/projects | 
[**ee_cloud_backend_projects_validate**](EeCloudBackendProjectsApi.md#ee_cloud_backend_projects_validate) | **POST** /cloud/backend/projects/validate | 



## ee_cloud_backend_projects_create

> crate::models::EeCloudBackendProjectsCreateResponse ee_cloud_backend_projects_create(ee_cloud_backend_projects_create_request)


Creates a new OpenGB project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ee_cloud_backend_projects_create_request** | [**EeCloudBackendProjectsCreateRequest**](EeCloudBackendProjectsCreateRequest.md) |  | [required] |

### Return type

[**crate::models::EeCloudBackendProjectsCreateResponse**](EeCloudBackendProjectsCreateResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_cloud_backend_projects_get

> crate::models::EeCloudBackendProjectsGetResponse ee_cloud_backend_projects_get(project_id, watch_index)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** |  | [required] |
**watch_index** | Option<**String**> | A query parameter denoting the requests watch index. |  |

### Return type

[**crate::models::EeCloudBackendProjectsGetResponse**](EeCloudBackendProjectsGetResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_cloud_backend_projects_list

> crate::models::EeCloudBackendProjectsListResponse ee_cloud_backend_projects_list(watch_index)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**watch_index** | Option<**String**> | A query parameter denoting the requests watch index. |  |

### Return type

[**crate::models::EeCloudBackendProjectsListResponse**](EeCloudBackendProjectsListResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_cloud_backend_projects_validate

> crate::models::EeCloudBackendProjectsValidateResponse ee_cloud_backend_projects_validate(ee_cloud_backend_projects_validate_request)


Validates information used to create a new project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ee_cloud_backend_projects_validate_request** | [**EeCloudBackendProjectsValidateRequest**](EeCloudBackendProjectsValidateRequest.md) |  | [required] |

### Return type

[**crate::models::EeCloudBackendProjectsValidateResponse**](EeCloudBackendProjectsValidateResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

