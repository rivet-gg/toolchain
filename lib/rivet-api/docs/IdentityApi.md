# \IdentityApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**identity_service_period_complete_avatar_upload**](IdentityApi.md#identity_service_period_complete_avatar_upload) | **POST** /identities/avatar-upload/{upload_id}/complete | 
[**identity_service_period_follow**](IdentityApi.md#identity_service_period_follow) | **POST** /identities/{identity_id}/follow | 
[**identity_service_period_get_handles**](IdentityApi.md#identity_service_period_get_handles) | **GET** /identities/batch/handle | 
[**identity_service_period_get_profile**](IdentityApi.md#identity_service_period_get_profile) | **GET** /identities/{identity_id}/profile | 
[**identity_service_period_get_self_profile**](IdentityApi.md#identity_service_period_get_self_profile) | **GET** /identities/self/profile | 
[**identity_service_period_get_summaries**](IdentityApi.md#identity_service_period_get_summaries) | **GET** /identities/batch/summary | 
[**identity_service_period_list_followers**](IdentityApi.md#identity_service_period_list_followers) | **GET** /identities/{identity_id}/followers | 
[**identity_service_period_list_friends**](IdentityApi.md#identity_service_period_list_friends) | **GET** /identities/self/friends | 
[**identity_service_period_list_mutual_friends**](IdentityApi.md#identity_service_period_list_mutual_friends) | **GET** /identities/{identity_id}/mutual-friends | 
[**identity_service_period_prepare_avatar_upload**](IdentityApi.md#identity_service_period_prepare_avatar_upload) | **POST** /identities/avatar-upload/prepare | 
[**identity_service_period_remove_game_activity**](IdentityApi.md#identity_service_period_remove_game_activity) | **DELETE** /identities/self/activity | 
[**identity_service_period_report**](IdentityApi.md#identity_service_period_report) | **POST** /identities/{identity_id}/report | 
[**identity_service_period_search**](IdentityApi.md#identity_service_period_search) | **GET** /identities/search | 
[**identity_service_period_set_game_activity**](IdentityApi.md#identity_service_period_set_game_activity) | **POST** /identities/self/activity | 
[**identity_service_period_setup**](IdentityApi.md#identity_service_period_setup) | **POST** /identities | 
[**identity_service_period_signup_for_beta**](IdentityApi.md#identity_service_period_signup_for_beta) | **POST** /identities/self/beta-signup | 
[**identity_service_period_unfollow**](IdentityApi.md#identity_service_period_unfollow) | **DELETE** /identities/{identity_id}/follow | 
[**identity_service_period_update_profile**](IdentityApi.md#identity_service_period_update_profile) | **POST** /identities/self/profile | 
[**identity_service_period_update_status**](IdentityApi.md#identity_service_period_update_status) | **POST** /identities/identities/self/status | 
[**identity_service_period_validate_profile**](IdentityApi.md#identity_service_period_validate_profile) | **POST** /identities/self/profile/validate | 



## identity_service_period_complete_avatar_upload

> identity_service_period_complete_avatar_upload(upload_id)


Completes an avatar image upload. Must be called after the file upload process completes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**upload_id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identity_service_period_follow

> identity_service_period_follow(identity_id)


Follows the given identity. In order for identities to be \"friends\", the other identity has to also follow this identity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identity_service_period_get_handles

> crate::models::GetHandlesOutput identity_service_period_get_handles(identity_ids)


Fetches a list of identity handles.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_ids** | **String** |  | [required] |

### Return type

[**crate::models::GetHandlesOutput**](GetHandlesOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identity_service_period_get_profile

> crate::models::GetProfileOutput identity_service_period_get_profile(identity_id, watch_index)


Fetches an identity profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_id** | **uuid::Uuid** |  | [required] |
**watch_index** | Option<**String**> |  |  |

### Return type

[**crate::models::GetProfileOutput**](GetProfileOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identity_service_period_get_self_profile

> crate::models::GetProfileOutput identity_service_period_get_self_profile(watch_index)


Fetches the current identity's profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**watch_index** | Option<**String**> |  |  |

### Return type

[**crate::models::GetProfileOutput**](GetProfileOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identity_service_period_get_summaries

> crate::models::GetSummariesOutput identity_service_period_get_summaries(identity_ids)


Fetches a list of identity summaries.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_ids** | **String** |  | [required] |

### Return type

[**crate::models::GetSummariesOutput**](GetSummariesOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identity_service_period_list_followers

> crate::models::ListFollowersOutput identity_service_period_list_followers(identity_id, anchor, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_id** | **uuid::Uuid** |  | [required] |
**anchor** | Option<**String**> |  |  |
**limit** | Option<**String**> | Range is between 1 and 32 (inclusive). |  |

### Return type

[**crate::models::ListFollowersOutput**](ListFollowersOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identity_service_period_list_friends

> crate::models::ListFriendsOutput identity_service_period_list_friends(anchor, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anchor** | Option<**String**> |  |  |
**limit** | Option<**String**> | Range is between 1 and 32 (inclusive). |  |

### Return type

[**crate::models::ListFriendsOutput**](ListFriendsOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identity_service_period_list_mutual_friends

> crate::models::ListMutualFriendsOutput identity_service_period_list_mutual_friends(identity_id, anchor, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_id** | **uuid::Uuid** |  | [required] |
**anchor** | Option<**String**> |  |  |
**limit** | Option<**String**> | Range is between 1 and 32 (inclusive). |  |

### Return type

[**crate::models::ListMutualFriendsOutput**](ListMutualFriendsOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identity_service_period_prepare_avatar_upload

> crate::models::PrepareAvatarUploadOutput identity_service_period_prepare_avatar_upload(identity_service_prepare_avatar_upload_request)


Prepares an avatar image upload. Complete upload with `CompleteIdentityAvatarUpload`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_service_prepare_avatar_upload_request** | [**IdentityServicePrepareAvatarUploadRequest**](IdentityServicePrepareAvatarUploadRequest.md) |  | [required] |

### Return type

[**crate::models::PrepareAvatarUploadOutput**](PrepareAvatarUploadOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identity_service_period_remove_game_activity

> identity_service_period_remove_game_activity()


Removes the current identity's game activity.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identity_service_period_report

> identity_service_period_report(identity_id, identity_service_report_request)


Creates an abuse report for an identity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_id** | **uuid::Uuid** |  | [required] |
**identity_service_report_request** | [**IdentityServiceReportRequest**](IdentityServiceReportRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identity_service_period_search

> crate::models::SearchOutput identity_service_period_search(query, anchor, limit)


Fuzzy search for identities.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | The query to match identity display names and account numbers against. | [required] |
**anchor** | Option<**String**> | How many identities to offset the search by. |  |
**limit** | Option<**i32**> | Amount of identities to return. Must be between 1 and 32 inclusive. |  |

### Return type

[**crate::models::SearchOutput**](SearchOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identity_service_period_set_game_activity

> identity_service_period_set_game_activity(identity_service_set_game_activity_request)


Sets the current identity's game activity. This activity will automatically be removed when the identity goes offline.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_service_set_game_activity_request** | [**IdentityServiceSetGameActivityRequest**](IdentityServiceSetGameActivityRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identity_service_period_setup

> crate::models::SetupOutput identity_service_period_setup(identity_link_token)


Gets or creates an identity. Passing an existing identity token in the body refreshes the token. Temporary Accounts Until the identity is linked with the Rivet Hub (see `PrepareGameLink`), this identity will be temporary but still behave like all other identities. This is intended to allow users to play the game without signing up while still having the benefits of having an account. When they are ready to save their account, they should be instructed to link their account (see `PrepareGameLink`). Storing Token `identity_token` should be stored in some form of persistent storage. The token should be read from storage and passed to `Setup` every time the client starts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_link_token** | **String** |  | [required] |

### Return type

[**crate::models::SetupOutput**](SetupOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identity_service_period_signup_for_beta

> identity_service_period_signup_for_beta(identity_service_signup_for_beta_request)


Submits a beta signup form.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_service_signup_for_beta_request** | [**IdentityServiceSignupForBetaRequest**](IdentityServiceSignupForBetaRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identity_service_period_unfollow

> identity_service_period_unfollow(identity_id)


Unfollows the given identity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identity_service_period_update_profile

> identity_service_period_update_profile(identity_service_update_profile_request)


Updates profile of the current identity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_service_update_profile_request** | [**IdentityServiceUpdateProfileRequest**](IdentityServiceUpdateProfileRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identity_service_period_update_status

> identity_service_period_update_status(identity_service_update_status_request)


Updates the current identity's status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_service_update_status_request** | [**IdentityServiceUpdateStatusRequest**](IdentityServiceUpdateStatusRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identity_service_period_validate_profile

> identity_service_period_validate_profile(identity_service_update_profile_request)


Validate contents of identity profile. Use to provide immediate feedback on profile changes before committing them.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_service_update_profile_request** | [**IdentityServiceUpdateProfileRequest**](IdentityServiceUpdateProfileRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

