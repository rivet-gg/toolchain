# CloudSvcPerf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**svc_name** | **String** | The name of the service. | 
**ts** | **String** | RFC3339 timestamp. | 
**duration** | Option<**f64**> | Unsigned 64 bit integer. | [optional]
**req_id** | Option<**String**> | A universally unique identifier. | [optional]
**spans** | [**Vec<crate::models::CloudLogsPerfSpan>**](CloudLogsPerfSpan.md) | A list of performance spans. | 
**marks** | [**Vec<crate::models::CloudLogsPerfMark>**](CloudLogsPerfMark.md) | A list of performance marks. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


