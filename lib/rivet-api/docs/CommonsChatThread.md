# CommonsChatThread

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**thread_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**create_ts** | **String** | RFC3339 timestamp | 
**topic** | [**crate::models::CommonsChatTopic**](CommonsChatTopic.md) |  | 
**tail_message** | Option<[**crate::models::CommonsChatMessage**](CommonsChatMessage.md)> |  | [optional]
**last_read_ts** | **String** | RFC3339 timestamp | 
**unread_count** | **i32** |  | 
**external** | [**crate::models::CommonsChatThreadExternalLinks**](CommonsChatThreadExternalLinks.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


