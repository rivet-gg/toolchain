# \CloudLogsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**logs_service_period_get_ray_perf_logs**](CloudLogsApi.md#logs_service_period_get_ray_perf_logs) | **GET** /rays/{ray_id}/perf | 



## logs_service_period_get_ray_perf_logs

> crate::models::CloudGetRayPerfLogsOutput logs_service_period_get_ray_perf_logs(ray_id)


Returns performance information about a Rivet Ray.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ray_id** | **String** | A universally unique identifier. | [required] |

### Return type

[**crate::models::CloudGetRayPerfLogsOutput**](CloudGetRayPerfLogsOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

