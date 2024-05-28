# \EeCloudOpengbProjectsApi

All URIs are relative to *https://api.rivet.gg*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ee_cloud_opengb_projects_create**](EeCloudOpengbProjectsApi.md#ee_cloud_opengb_projects_create) | **POST** /cloud/opengb/projects | 
[**ee_cloud_opengb_projects_get**](EeCloudOpengbProjectsApi.md#ee_cloud_opengb_projects_get) | **GET** /cloud/opengb/projects/{project_id} | 
[**ee_cloud_opengb_projects_list**](EeCloudOpengbProjectsApi.md#ee_cloud_opengb_projects_list) | **GET** /cloud/opengb/projects | 
[**ee_cloud_opengb_projects_validate**](EeCloudOpengbProjectsApi.md#ee_cloud_opengb_projects_validate) | **POST** /cloud/opengb/projects/validate | 



## ee_cloud_opengb_projects_create

> crate::models::EeCloudOpengbProjectsCreateResponse ee_cloud_opengb_projects_create(ee_cloud_opengb_projects_create_request)


Creates a new OpenGB project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ee_cloud_opengb_projects_create_request** | [**EeCloudOpengbProjectsCreateRequest**](EeCloudOpengbProjectsCreateRequest.md) |  | [required] |

### Return type

[**crate::models::EeCloudOpengbProjectsCreateResponse**](EeCloudOpengbProjectsCreateResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_cloud_opengb_projects_get

> crate::models::EeCloudOpengbProjectsGetResponse ee_cloud_opengb_projects_get(project_id, watch_index)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** |  | [required] |
**watch_index** | Option<**String**> | A query parameter denoting the requests watch index. |  |

### Return type

[**crate::models::EeCloudOpengbProjectsGetResponse**](EeCloudOpengbProjectsGetResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_cloud_opengb_projects_list

> crate::models::EeCloudOpengbProjectsListResponse ee_cloud_opengb_projects_list(watch_index)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**watch_index** | Option<**String**> | A query parameter denoting the requests watch index. |  |

### Return type

[**crate::models::EeCloudOpengbProjectsListResponse**](EeCloudOpengbProjectsListResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_cloud_opengb_projects_validate

> crate::models::EeCloudOpengbProjectsValidateResponse ee_cloud_opengb_projects_validate(ee_cloud_opengb_projects_validate_request)


Validates information used to create a new project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ee_cloud_opengb_projects_validate_request** | [**EeCloudOpengbProjectsValidateRequest**](EeCloudOpengbProjectsValidateRequest.md) |  | [required] |

### Return type

[**crate::models::EeCloudOpengbProjectsValidateResponse**](EeCloudOpengbProjectsValidateResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

