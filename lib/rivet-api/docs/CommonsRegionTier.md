# CommonsRegionTier

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tier_name_id** | **String** | A human readable short identifier used to references resources. Different than a `rivet.common#Uuid` because this is intended to be human readable. Different than `rivet.common#DisplayName` because this should not include special characters and be short. | 
**rivet_cores_numerator** | Option<**f64**> | Together with the denominator, denotes the portion of the CPU a given server uses. | [optional]
**rivet_cores_denominator** | Option<**f64**> | Together with the numerator, denotes the portion of the CPU a given server uses. | [optional]
**cpu** | Option<**f64**> | CPU frequency (MHz). | [optional]
**memory** | Option<**f64**> | Allocated memory (MB). | [optional]
**disk** | Option<**f64**> | Allocated disk space (MB). | [optional]
**bandwidth** | Option<**f64**> | Internet bandwidth (MB). | [optional]
**price_per_second** | Option<**f64**> | Price billed for every second this server is running (in quadrillionth USD, 1,000,000,000,000 = $1.00). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


