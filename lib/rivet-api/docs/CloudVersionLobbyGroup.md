# CloudVersionLobbyGroup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**max_players_direct** | Option<**f64**> | Unsigned 32 bit integer. | [optional]
**max_players_normal** | Option<**f64**> | Unsigned 32 bit integer. | [optional]
**max_players_party** | Option<**f64**> | Unsigned 32 bit integer. | [optional]
**name_id** | **String** | A human readable short identifier used to references resources. Different than a `rivet.common#Uuid` because this is intended to be human readable. Different than `rivet.common#DisplayName` because this should not include special characters and be short. | 
**regions** | [**Vec<crate::models::CloudVersionLobbyGroupRegion>**](CloudVersionLobbyGroupRegion.md) | A list of game mode regions. | 
**runtime** | [**crate::models::CloudVersionLobbyGroupRuntime**](CloudVersionLobbyGroupRuntime.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


