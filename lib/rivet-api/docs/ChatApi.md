# \ChatApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**chat_get_thread_history**](ChatApi.md#chat_get_thread_history) | **GET** /threads/{thread_id}/history | 
[**chat_get_thread_topic**](ChatApi.md#chat_get_thread_topic) | **GET** /threads/{thread_id}/topic | 
[**chat_send_message**](ChatApi.md#chat_send_message) | **POST** /messages | 
[**chat_set_thread_read**](ChatApi.md#chat_set_thread_read) | **POST** /threads/{thread_id}/read | 
[**chat_set_typing_status**](ChatApi.md#chat_set_typing_status) | **PUT** /threads/{thread_id}/typing-status | 
[**chat_watch_thread**](ChatApi.md#chat_watch_thread) | **GET** /threads/{thread_id}/live | 



## chat_get_thread_history

> crate::models::GetThreadHistoryOutput chat_get_thread_history(thread_id, count, ts, query_direction)


Returns message history for a given thread in a certain direction. Defaults to querying messages before ts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thread_id** | **String** | A universally unique identifier. | [required] |
**count** | **f64** | How many messages to collect in each direction. If querying `rivet.api.chat.common#QueryDirection$before_and_after`, `rivet.api.chat.common#QueryDirection$chat_messages` will be `count * 2`. | [required] |
**ts** | Option<**String**> | RFC3339 timestamp. |  |
**query_direction** | Option<**String**> | Represents which direction to query messages from relative to the given timestamp. |  |

### Return type

[**crate::models::GetThreadHistoryOutput**](GetThreadHistoryOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_get_thread_topic

> crate::models::GetThreadTopicOutput chat_get_thread_topic(thread_id)


Fetches the topic of a thread.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thread_id** | **String** | A universally unique identifier. | [required] |

### Return type

[**crate::models::GetThreadTopicOutput**](GetThreadTopicOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_send_message

> crate::models::SendMessageOutput chat_send_message(send_message_input)


Sends a chat message to a given topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**send_message_input** | [**SendMessageInput**](SendMessageInput.md) |  | [required] |

### Return type

[**crate::models::SendMessageOutput**](SendMessageOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_set_thread_read

> chat_set_thread_read(thread_id, set_thread_read_input)


Updates the current identity's last read timestamp in the given thread.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thread_id** | **String** | A universally unique identifier. | [required] |
**set_thread_read_input** | [**SetThreadReadInput**](SetThreadReadInput.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_set_typing_status

> chat_set_typing_status(thread_id, set_typing_status_input)


Updates the current identity's typing status in the given thread.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thread_id** | **String** | A universally unique identifier. | [required] |
**set_typing_status_input** | [**SetTypingStatusInput**](SetTypingStatusInput.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_watch_thread

> crate::models::WatchThreadOutput chat_watch_thread(thread_id, watch_index)


Fetches all relevant changes from a thread that have happened since the given watch index.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thread_id** | **String** | A universally unique identifier. | [required] |
**watch_index** | Option<**String**> | A query parameter denoting the requests watch index. |  |

### Return type

[**crate::models::WatchThreadOutput**](WatchThreadOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

