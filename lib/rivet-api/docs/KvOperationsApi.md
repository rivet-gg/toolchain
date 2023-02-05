# \KvOperationsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**operations_delete**](KvOperationsApi.md#operations_delete) | **DELETE** /entries | 
[**operations_get**](KvOperationsApi.md#operations_get) | **GET** /entries | 
[**operations_put**](KvOperationsApi.md#operations_put) | **PUT** /entries | 



## operations_delete

> operations_delete(key, namespace_id)


Deletes a key-value entry by key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | A string reprenting a key in the key-value database. Key path components are split by a slash (e.g. `a/b/c` has the path components `[\"a\", \"b\", \"c\"]`). Slashes can be escaped by using a forward slash (e.g. `a/b/c/d` has the path components `[\"a\", \"b/c\", \"d\"]`). See `rivet.api.kv.common#KeyComponents` for the structure of a `rivet.api.kv.common#Key` split by `/`. | [required] |
**namespace_id** | Option<**String**> | A universally unique identifier. |  |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## operations_get

> crate::models::KvGetOutput operations_get(key, watch_index, namespace_id)


Returns a specific key-value entry by key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | A string reprenting a key in the key-value database. Key path components are split by a slash (e.g. `a/b/c` has the path components `[\"a\", \"b\", \"c\"]`). Slashes can be escaped by using a forward slash (e.g. `a/b/c/d` has the path components `[\"a\", \"b/c\", \"d\"]`). See `rivet.api.kv.common#KeyComponents` for the structure of a `rivet.api.kv.common#Key` split by `/`. | [required] |
**watch_index** | Option<**String**> | A query parameter denoting the requests watch index. |  |
**namespace_id** | Option<**String**> | A universally unique identifier. |  |

### Return type

[**crate::models::KvGetOutput**](KvGetOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## operations_put

> operations_put(kv_put_input)


Puts (sets or overwrites) a key-value entry by key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**kv_put_input** | [**KvPutInput**](KvPutInput.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

