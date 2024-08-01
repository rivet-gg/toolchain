# \EeCloudGamesBillingApi

All URIs are relative to *https://api.rivet.gg*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ee_cloud_games_billing_get**](EeCloudGamesBillingApi.md#ee_cloud_games_billing_get) | **GET** /cloud/games/{game_id}/billing | 
[**ee_cloud_games_billing_update_plan**](EeCloudGamesBillingApi.md#ee_cloud_games_billing_update_plan) | **PUT** /cloud/games/{game_id}/billing/plan | 



## ee_cloud_games_billing_get

> crate::models::EeCloudGamesBillingGetResponse ee_cloud_games_billing_get(game_id, watch_index)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **uuid::Uuid** |  | [required] |
**watch_index** | Option<**String**> |  |  |

### Return type

[**crate::models::EeCloudGamesBillingGetResponse**](EeCloudGamesBillingGetResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_cloud_games_billing_update_plan

> ee_cloud_games_billing_update_plan(game_id, ee_cloud_games_billing_update_plan_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **uuid::Uuid** |  | [required] |
**ee_cloud_games_billing_update_plan_request** | [**EeCloudGamesBillingUpdatePlanRequest**](EeCloudGamesBillingUpdatePlanRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

