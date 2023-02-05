# CloudLogsLobbySummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**create_ts** | **String** | RFC3339 timestamp. | 
**lobby_group_name_id** | **String** | A human readable short identifier used to references resources. Different than a `rivet.common#Uuid` because this is intended to be human readable. Different than `rivet.common#DisplayName` because this should not include special characters and be short. | 
**lobby_id** | **String** | A universally unique identifier. | 
**namespace_id** | **String** | A universally unique identifier. | 
**ready_ts** | Option<**String**> | RFC3339 timestamp. | [optional]
**region_id** | **String** | A universally unique identifier. | 
**start_ts** | Option<**String**> | RFC3339 timestamp. | [optional]
**status** | [**crate::models::CloudLogsLobbyStatus**](CloudLogsLobbyStatus.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


