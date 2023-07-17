# CloudVersionEngineUnrealConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**base_docker_image** | **String** | Docker image to use for running the server locally & building the server to run on Rivet. See versions [here](https://github.com/orgs/EpicGames/packages/container/package/unreal-engine). _Configures Rivet CLI behavior. Has no effect on server behavior._ | 
**game_module** | **String** | Name of the Unreal module that holds the game code. This is usually the value of `$.Modules[0].Name` in the file `MyProject.unproject`. _Configures Rivet CLI behavior. Has no effect on server behavior._ | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


