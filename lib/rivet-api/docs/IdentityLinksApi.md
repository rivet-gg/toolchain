# \IdentityLinksApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**links_cancel**](IdentityLinksApi.md#links_cancel) | **POST** /game-links/cancel | 
[**links_complete**](IdentityLinksApi.md#links_complete) | **POST** /game-links/complete | 
[**links_get**](IdentityLinksApi.md#links_get) | **GET** /game-links | 
[**links_prepare**](IdentityLinksApi.md#links_prepare) | **POST** /game-links | 



## links_cancel

> links_cancel(links_cancel_request)


Cancels a game link. It can no longer be used to link after cancellation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**links_cancel_request** | [**LinksCancelRequest**](LinksCancelRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## links_complete

> links_complete(links_cancel_request)


Completes a game link process and returns whether or not the link is valid.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**links_cancel_request** | [**LinksCancelRequest**](LinksCancelRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## links_get

> crate::models::IdentityGetGameLinkOutput links_get(identity_link_token, watch_index)


Returns the current status of a linking process. Once `status` is `complete`, the identity's profile should be fetched again since they may have switched accounts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_link_token** | **String** |  | [required] |
**watch_index** | **String** |  | [required] |

### Return type

[**crate::models::IdentityGetGameLinkOutput**](IdentityGetGameLinkOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## links_prepare

> crate::models::IdentityPrepareGameLinkOutput links_prepare()


Begins the process for linking an identity with the Rivet Hub. # Importance of Linking Identities When an identity is created via `rivet.api.identity#SetupIdentity`, the identity is temporary and is not shared with other games the user plays. In order to make the identity permanent and synchronize the identity with other games, the identity must be linked with the hub. # Linking Process The linking process works by opening `identity_link_url` in a browser then polling `rivet.api.identity#GetGameLink` to wait for it to complete. This is designed to be as flexible as possible so `identity_link_url` can be opened on any device. For example, when playing a console game, the user can scan a QR code for `identity_link_url` to authenticate on their phone. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::IdentityPrepareGameLinkOutput**](IdentityPrepareGameLinkOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

