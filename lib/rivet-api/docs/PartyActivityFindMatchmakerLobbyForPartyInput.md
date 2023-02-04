# PartyActivityFindMatchmakerLobbyForPartyInput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**game_modes** | **Vec<String>** | Game modes to match lobbies against. | 
**regions** | Option<**Vec<String>**> | Regions to match lobbies against. If not specified, the optimal region will be determined and will attempt to find lobbies in that region. | [optional]
**prevent_auto_create_lobby** | Option<**bool**> | Prevents a new lobby from being created when finding a lobby. If no lobby is found, `MATCHMAKER_LOBBY_NOT_FOUND` will be returned. | [optional]
**captcha** | Option<[**crate::models::CommonsCaptchaConfig**](CommonsCaptchaConfig.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


