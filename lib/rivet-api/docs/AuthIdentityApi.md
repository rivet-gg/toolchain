# \AuthIdentityApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**auth_identity_complete_email_verification**](AuthIdentityApi.md#auth_identity_complete_email_verification) | **POST** /identity/email/complete-verification | 
[**auth_identity_start_email_verification**](AuthIdentityApi.md#auth_identity_start_email_verification) | **POST** /identity/email/start-verification | 



## auth_identity_complete_email_verification

> crate::models::AuthCompleteEmailVerificationOutput auth_identity_complete_email_verification(auth_complete_email_verification_input)


Completes the email verification process.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_complete_email_verification_input** | [**AuthCompleteEmailVerificationInput**](AuthCompleteEmailVerificationInput.md) |  | [required] |

### Return type

[**crate::models::AuthCompleteEmailVerificationOutput**](AuthCompleteEmailVerificationOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_identity_start_email_verification

> crate::models::AuthStartEmailVerificationOutput auth_identity_start_email_verification(auth_start_email_verification_input)


Starts the verification process for linking an email to your identity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_start_email_verification_input** | [**AuthStartEmailVerificationInput**](AuthStartEmailVerificationInput.md) |  | [required] |

### Return type

[**crate::models::AuthStartEmailVerificationOutput**](AuthStartEmailVerificationOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

