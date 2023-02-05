# \KvBatchOperationsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**batch_operations_delete_batch**](KvBatchOperationsApi.md#batch_operations_delete_batch) | **DELETE** /entries/batch | 
[**batch_operations_get_batch**](KvBatchOperationsApi.md#batch_operations_get_batch) | **GET** /entries/batch | 
[**batch_operations_put_batch**](KvBatchOperationsApi.md#batch_operations_put_batch) | **PUT** /entries/batch | 



## batch_operations_delete_batch

> batch_operations_delete_batch(namespace_id)


Deletes multiple key-value entries by key(s).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace_id** | Option<**String**> | A universally unique identifier. |  |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## batch_operations_get_batch

> crate::models::KvGetBatchOutput batch_operations_get_batch(watch_index, namespace_id)


Gets multiple key-value entries by key(s).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**watch_index** | Option<**String**> | A query parameter denoting the requests watch index. |  |
**namespace_id** | Option<**String**> | A universally unique identifier. |  |

### Return type

[**crate::models::KvGetBatchOutput**](KvGetBatchOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## batch_operations_put_batch

> batch_operations_put_batch(kv_put_batch_input)


Puts (sets or overwrites) multiple key-value entries by key(s).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**kv_put_batch_input** | [**KvPutBatchInput**](KvPutBatchInput.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

