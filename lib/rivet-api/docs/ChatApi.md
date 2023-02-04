# \ChatApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**chat_service_period_get_thread_topic**](ChatApi.md#chat_service_period_get_thread_topic) | **GET** /threads/{thread_id}/topic | 
[**chat_service_period_send_chat_message**](ChatApi.md#chat_service_period_send_chat_message) | **POST** /messages | 
[**chat_service_period_set_thread_read**](ChatApi.md#chat_service_period_set_thread_read) | **POST** /threads/{thread_id}/read | 
[**chat_service_period_set_typing_status**](ChatApi.md#chat_service_period_set_typing_status) | **PUT** /threads/{thread_id}/typing-status | 
[**chat_service_period_watch_thread**](ChatApi.md#chat_service_period_watch_thread) | **GET** /threads/{thread_id}/live | 



## chat_service_period_get_thread_topic

> crate::models::GetThreadTopicOutput chat_service_period_get_thread_topic(thread_id)


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


## chat_service_period_send_chat_message

> crate::models::SendChatMessageOutput chat_service_period_send_chat_message(send_chat_message_input)


Sends a chat message to a given topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**send_chat_message_input** | [**SendChatMessageInput**](SendChatMessageInput.md) |  | [required] |

### Return type

[**crate::models::SendChatMessageOutput**](SendChatMessageOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_service_period_set_thread_read

> chat_service_period_set_thread_read(thread_id, set_thread_read_input)


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


## chat_service_period_set_typing_status

> chat_service_period_set_typing_status(thread_id, set_typing_status_input)


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


## chat_service_period_watch_thread

> crate::models::WatchThreadOutput chat_service_period_watch_thread(thread_id, watch_index)


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

