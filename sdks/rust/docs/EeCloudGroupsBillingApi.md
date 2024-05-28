# \EeCloudGroupsBillingApi

All URIs are relative to *https://api.rivet.gg*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ee_cloud_groups_billing_create_stripe_portal_session**](EeCloudGroupsBillingApi.md#ee_cloud_groups_billing_create_stripe_portal_session) | **POST** /cloud/groups/{group_id}/billing/stripe-portal-session | 
[**ee_cloud_groups_billing_get**](EeCloudGroupsBillingApi.md#ee_cloud_groups_billing_get) | **GET** /cloud/groups/{group_id}/billing | 



## ee_cloud_groups_billing_create_stripe_portal_session

> crate::models::EeCloudGroupsBillingCreateStripePortalSessionResponse ee_cloud_groups_billing_create_stripe_portal_session(group_id, ee_cloud_groups_billing_create_stripe_portal_session_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **uuid::Uuid** |  | [required] |
**ee_cloud_groups_billing_create_stripe_portal_session_request** | [**EeCloudGroupsBillingCreateStripePortalSessionRequest**](EeCloudGroupsBillingCreateStripePortalSessionRequest.md) |  | [required] |

### Return type

[**crate::models::EeCloudGroupsBillingCreateStripePortalSessionResponse**](EeCloudGroupsBillingCreateStripePortalSessionResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ee_cloud_groups_billing_get

> crate::models::EeCloudGroupsBillingGetBillingResponse ee_cloud_groups_billing_get(group_id, watch_index)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **uuid::Uuid** |  | [required] |
**watch_index** | Option<**String**> |  |  |

### Return type

[**crate::models::EeCloudGroupsBillingGetBillingResponse**](EeCloudGroupsBillingGetBillingResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

