# \UploadApi

All URIs are relative to *https://api-cloud.rivet.gg/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**complete_upload**](UploadApi.md#complete_upload) | **post** /uploads/{upload_id}/complete | 



## complete_upload

> serde_json::Value complete_upload(upload_id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**upload_id** | [**String**](.md) |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

