# CloudRegionTier

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bandwidth** | Option<**f64**> | Internet bandwidth (MB). | [optional]
**cpu** | Option<**f64**> | CPU frequency (MHz). | [optional]
**disk** | Option<**f64**> | Allocated disk space (MB). | [optional]
**memory** | Option<**f64**> | Allocated memory (MB). | [optional]
**price_per_second** | Option<**f64**> | Price billed for every second this server is running (in quadrillionth USD, 1,000,000,000,000 = $1.00). | [optional]
**rivet_cores_denominator** | Option<**f64**> | Together with the numerator, denotes the portion of the CPU a given server uses. | [optional]
**rivet_cores_numerator** | Option<**f64**> | Together with the denominator, denotes the portion of the CPU a given server uses. | [optional]
**tier_name_id** | **String** | A human readable short identifier used to references resources. Different than a `rivet.common#Uuid` because this is intended to be human readable. Different than `rivet.common#DisplayName` because this should not include special characters and be short. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


