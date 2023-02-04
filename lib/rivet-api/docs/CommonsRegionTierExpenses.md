# CommonsRegionTierExpenses

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**namespace_id** | **String** | A universally unique identifier. | 
**region_id** | **String** | A universally unique identifier. | 
**tier_name_id** | **String** | A human readable short identifier used to references resources. Different than a `rivet.common#Uuid` because this is intended to be human readable. Different than `rivet.common#DisplayName` because this should not include special characters and be short. | 
**lobby_group_name_id** | **String** | A human readable short identifier used to references resources. Different than a `rivet.common#Uuid` because this is intended to be human readable. Different than `rivet.common#DisplayName` because this should not include special characters and be short. | 
**uptime** | Option<**f64**> | How long a region tier has been active (in milliseconds). | [optional]
**expenses** | Option<**f64**> | Amount of expenses for this region tier (in hundred-thousandths USD, 100,000 = $1.00). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


