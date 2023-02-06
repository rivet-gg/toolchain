# \CloudGroupsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cloud_groups_convert_group**](CloudGroupsApi.md#cloud_groups_convert_group) | **POST** /groups/{group_id}/convert | 
[**cloud_groups_get_group_billing**](CloudGroupsApi.md#cloud_groups_get_group_billing) | **GET** /groups/{group_id}/billing | 
[**cloud_groups_get_group_invoices_list**](CloudGroupsApi.md#cloud_groups_get_group_invoices_list) | **GET** /groups/{group_id}/billing/invoices | 
[**cloud_groups_get_group_payments_list**](CloudGroupsApi.md#cloud_groups_get_group_payments_list) | **GET** /groups/{group_id}/billing/payments | 
[**cloud_groups_get_group_transfers_list**](CloudGroupsApi.md#cloud_groups_get_group_transfers_list) | **GET** /groups/{group_id}/billing/transfers | 
[**cloud_groups_group_billing_checkout**](CloudGroupsApi.md#cloud_groups_group_billing_checkout) | **POST** /groups/{group_id}/checkout | 
[**cloud_groups_validate_group**](CloudGroupsApi.md#cloud_groups_validate_group) | **POST** /groups/validate | 



## cloud_groups_convert_group

> cloud_groups_convert_group(group_id)


Converts the given group into a developer group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | A universally unique identifier. | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_groups_get_group_billing

> crate::models::CloudGetGroupBillingOutput cloud_groups_get_group_billing(group_id, query_start, query_end)


Returns billing information for the given group over the given query time span.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | A universally unique identifier. | [required] |
**query_start** | Option<**f64**> | Unsigned 64 bit integer. |  |
**query_end** | Option<**f64**> | Unsigned 64 bit integer. |  |

### Return type

[**crate::models::CloudGetGroupBillingOutput**](CloudGetGroupBillingOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_groups_get_group_invoices_list

> crate::models::CloudGetGroupInvoicesListOutput cloud_groups_get_group_invoices_list(group_id, anchor, limit)


Returns a list of invoices for the given group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | A universally unique identifier. | [required] |
**anchor** | Option<**String**> | The pagination anchor. Set to the returned anchor of this endpoint to receive the next set of items. |  |
**limit** | Option<**f64**> | Amount of invoices to return. |  |

### Return type

[**crate::models::CloudGetGroupInvoicesListOutput**](CloudGetGroupInvoicesListOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_groups_get_group_payments_list

> crate::models::CloudGetGroupPaymentsListOutput cloud_groups_get_group_payments_list(group_id, start_payment_id)


Returns a list of payments for the given group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | A universally unique identifier. | [required] |
**start_payment_id** | Option<**String**> | The payment ID of the payment after which to start listing. |  |

### Return type

[**crate::models::CloudGetGroupPaymentsListOutput**](CloudGetGroupPaymentsListOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_groups_get_group_transfers_list

> crate::models::CloudGetGroupTransfersListOutput cloud_groups_get_group_transfers_list(group_id, start_transfer_id)


Returns a list of bank transfers for the given group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | A universally unique identifier. | [required] |
**start_transfer_id** | Option<**String**> | The transfer ID of the transfer after which to start listing. |  |

### Return type

[**crate::models::CloudGetGroupTransfersListOutput**](CloudGetGroupTransfersListOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_groups_group_billing_checkout

> crate::models::CloudGroupBillingCheckoutOutput cloud_groups_group_billing_checkout(group_id, cloud_group_billing_checkout_input)


Creates a checkout session for the given group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | A universally unique identifier. | [required] |
**cloud_group_billing_checkout_input** | [**CloudGroupBillingCheckoutInput**](CloudGroupBillingCheckoutInput.md) |  | [required] |

### Return type

[**crate::models::CloudGroupBillingCheckoutOutput**](CloudGroupBillingCheckoutOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cloud_groups_validate_group

> crate::models::CloudValidateGroupOutput cloud_groups_validate_group(cloud_validate_group_input)


Validates information used to create a new group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cloud_validate_group_input** | [**CloudValidateGroupInput**](CloudValidateGroupInput.md) |  | [required] |

### Return type

[**crate::models::CloudValidateGroupOutput**](CloudValidateGroupOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

