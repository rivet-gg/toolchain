# ActorLifecycle

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**durable** | Option<**bool**> | If true, the actor will try to reschedule itself automatically in the event of a crash or a datacenter failover. The actor will not reschedule if it exits successfully. | [optional]
**kill_timeout** | Option<**i64**> | The duration to wait for in milliseconds before killing the actor. This should be set to a safe default, and can be overridden during a DELETE request if needed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


