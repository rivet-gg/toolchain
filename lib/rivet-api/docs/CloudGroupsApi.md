# \CloudGroupsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cloud_groups_billing_checkout**](CloudGroupsApi.md#cloud_groups_billing_checkout) | **POST** /groups/{group_id}/checkout | 
[**cloud_groups_convert_group**](CloudGroupsApi.md#cloud_groups_convert_group) | **POST** /groups/{group_id}/convert | 
[**cloud_groups_get_billing**](CloudGroupsApi.md#cloud_groups_get_billing) | **GET** /groups/{group_id}/billing | 
[**cloud_groups_get_invoices_list**](CloudGroupsApi.md#cloud_groups_get_invoices_list) | **GET** /groups/{group_id}/billing/invoices | 
[**cloud_groups_get_payments_list**](CloudGroupsApi.md#cloud_groups_get_payments_list) | **GET** /groups/{group_id}/billing/payments | 
[**cloud_groups_get_transfers_list**](CloudGroupsApi.md#cloud_groups_get_transfers_list) | **GET** /groups/{group_id}/billing/transfers | 
[**cloud_groups_validate**](CloudGroupsApi.md#cloud_groups_validate) | **POST** /groups/validate | 



## cloud_groups_billing_checkout

> crate::models::CloudGroupBillingCheckoutResponse cloud_groups_billing_checkout(group_id, cloud_group_billing_checkout_request)


Creates a checkout session for the given group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **uuid::Uuid** |  | [required] |
**cloud_group_billing_checkout_request** | [**CloudGroupBillingCheckoutRequest**](CloudGroupBillingCheckoutRequest.md) |  | [required] |

### Return type

[**crate::models::CloudGroupBillingCheckoutResponse**](CloudGroupBillingCheckoutResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_groups_convert_group

> cloud_groups_convert_group(group_id)


Converts the given group into a developer group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_groups_get_billing

> crate::models::CloudGetBillingResponse cloud_groups_get_billing(group_id, query_start, query_end)


Returns billing information for the given group over the given query time span.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **uuid::Uuid** |  | [required] |
**query_start** | Option<**String**> |  |  |
**query_end** | Option<**String**> |  |  |

### Return type

[**crate::models::CloudGetBillingResponse**](CloudGetBillingResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_groups_get_invoices_list

> crate::models::CloudGetInvoicesListResponse cloud_groups_get_invoices_list(group_id, anchor, limit)


Returns a list of invoices for the given group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **uuid::Uuid** |  | [required] |
**anchor** | Option<**String**> | The pagination anchor. Set to the returned anchor of this endpoint to receive the next set of items. |  |
**limit** | Option<**i64**> | Amount of invoices to return. |  |

### Return type

[**crate::models::CloudGetInvoicesListResponse**](CloudGetInvoicesListResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_groups_get_payments_list

> crate::models::CloudGetPaymentsListResponse cloud_groups_get_payments_list(group_id, start_payment_id)


Returns a list of payments for the given group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **uuid::Uuid** |  | [required] |
**start_payment_id** | Option<**String**> | The payment ID of the payment after which to start listing. |  |

### Return type

[**crate::models::CloudGetPaymentsListResponse**](CloudGetPaymentsListResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_groups_get_transfers_list

> crate::models::CloudGetTransfersListResponse cloud_groups_get_transfers_list(group_id, start_transfer_id)


Returns a list of bank transfers for the given group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **uuid::Uuid** |  | [required] |
**start_transfer_id** | Option<**String**> | The transfer ID of the transfer after which to start listing. |  |

### Return type

[**crate::models::CloudGetTransfersListResponse**](CloudGetTransfersListResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_groups_validate

> crate::models::CloudValidateGroupResponse cloud_groups_validate(cloud_validate_group_request)


Validates information used to create a new group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cloud_validate_group_request** | [**CloudValidateGroupRequest**](CloudValidateGroupRequest.md) |  | [required] |

### Return type

[**crate::models::CloudValidateGroupResponse**](CloudValidateGroupResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

