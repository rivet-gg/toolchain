# CloudAnalyticsLobbySummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**create_ts** | **String** | RFC3339 timestamp. | 
**is_closed** | Option<**bool**> | Whether or not this lobby is in a closed state. | [optional]
**is_idle** | Option<**bool**> | Whether or not this lobby is idle. | [optional]
**is_outdated** | Option<**bool**> | Whether or not this lobby is outdated. | [optional]
**is_ready** | Option<**bool**> | Whether or not this lobby is ready. | [optional]
**lobby_group_id** | **String** | A universally unique identifier. | 
**lobby_group_name_id** | **String** | A human readable short identifier used to references resources. Different than a `rivet.common#Uuid` because this is intended to be human readable. Different than `rivet.common#DisplayName` because this should not include special characters and be short. | 
**lobby_id** | **String** | A universally unique identifier. | 
**max_players_direct** | Option<**i32**> | Unsigned 32 bit integer. | [optional]
**max_players_normal** | Option<**i32**> | Unsigned 32 bit integer. | [optional]
**max_players_party** | Option<**i32**> | Unsigned 32 bit integer. | [optional]
**region_id** | **String** | A universally unique identifier. | 
**registered_player_count** | Option<**i32**> | Unsigned 32 bit integer. | [optional]
**total_player_count** | Option<**i32**> | Unsigned 32 bit integer. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


