# \PartyPartiesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**parties_service_period_create**](PartyPartiesApi.md#parties_service_period_create) | **POST** /parties | 
[**parties_service_period_create_invite**](PartyPartiesApi.md#parties_service_period_create_invite) | **POST** /parties/self/invites | 
[**parties_service_period_get_from_invite**](PartyPartiesApi.md#parties_service_period_get_from_invite) | **GET** /invites | 
[**parties_service_period_get_profile**](PartyPartiesApi.md#parties_service_period_get_profile) | **GET** /parties/{party_id}/profile | 
[**parties_service_period_get_self_profile**](PartyPartiesApi.md#parties_service_period_get_self_profile) | **GET** /parties/self/profile | 
[**parties_service_period_get_self_summary**](PartyPartiesApi.md#parties_service_period_get_self_summary) | **GET** /parties/self/summary | 
[**parties_service_period_get_summary**](PartyPartiesApi.md#parties_service_period_get_summary) | **GET** /parties/{party_id}/summary | 
[**parties_service_period_join**](PartyPartiesApi.md#parties_service_period_join) | **POST** /parties/join | 
[**parties_service_period_kick_member**](PartyPartiesApi.md#parties_service_period_kick_member) | **POST** /parties/self/members/{identity_id}/kick | 
[**parties_service_period_leave**](PartyPartiesApi.md#parties_service_period_leave) | **POST** /parties/self/leave | 
[**parties_service_period_revoke_invite**](PartyPartiesApi.md#parties_service_period_revoke_invite) | **DELETE** /parties/self/invites/{invite_id} | 
[**parties_service_period_send_join_request**](PartyPartiesApi.md#parties_service_period_send_join_request) | **POST** /parties/{party_id}/join-request/send | 
[**parties_service_period_set_publicity**](PartyPartiesApi.md#parties_service_period_set_publicity) | **PUT** /parties/self/publicity | 
[**parties_service_period_transfer_ownership**](PartyPartiesApi.md#parties_service_period_transfer_ownership) | **POST** /parties/self/members/{identity_id}/transfer-ownership | 



## parties_service_period_create

> crate::models::PartyCreateOutput parties_service_period_create(party_create_input)


Creates a new party.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**party_create_input** | [**PartyCreateInput**](PartyCreateInput.md) |  | [required] |

### Return type

[**crate::models::PartyCreateOutput**](PartyCreateOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_service_period_create_invite

> crate::models::PartyCreateInviteOutput parties_service_period_create_invite(party_create_invite_input)


Creates a new party invite for the current identity's party. Identity must be the party leader.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**party_create_invite_input** | [**PartyCreateInviteInput**](PartyCreateInviteInput.md) |  | [required] |

### Return type

[**crate::models::PartyCreateInviteOutput**](PartyCreateInviteOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_service_period_get_from_invite

> crate::models::PartyGetInviteOutput parties_service_period_get_from_invite(token, alias)


Fetches a party based on a given invite.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | See `rivet.api.party#CreatedInvite$token`. |  |
**alias** | Option<**String**> | An alias used to join a given party. This alias must be unique for all invites for your game. Pass this alias to `rivet.api.party.common#CreatedInvite$alias` to consume the invite. |  |

### Return type

[**crate::models::PartyGetInviteOutput**](PartyGetInviteOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_service_period_get_profile

> crate::models::PartyGetProfileOutput parties_service_period_get_profile(party_id, watch_index)


Returns a party profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**party_id** | **String** | A universally unique identifier. | [required] |
**watch_index** | Option<**String**> | A query parameter denoting the requests watch index. |  |

### Return type

[**crate::models::PartyGetProfileOutput**](PartyGetProfileOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_service_period_get_self_profile

> crate::models::PartyGetSelfProfileOutput parties_service_period_get_self_profile(watch_index)


Returns a party profile for the party the current identity is a member of.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**watch_index** | Option<**String**> | A query parameter denoting the requests watch index. |  |

### Return type

[**crate::models::PartyGetSelfProfileOutput**](PartyGetSelfProfileOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_service_period_get_self_summary

> crate::models::PartyGetSelfSummaryOutput parties_service_period_get_self_summary(watch_index)


Returns a party summary for the party the current identity is a member of.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**watch_index** | Option<**String**> | A query parameter denoting the requests watch index. |  |

### Return type

[**crate::models::PartyGetSelfSummaryOutput**](PartyGetSelfSummaryOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_service_period_get_summary

> crate::models::PartyGetSummaryOutput parties_service_period_get_summary(party_id, watch_index)


Returns a party summary.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**party_id** | **String** | A universally unique identifier. | [required] |
**watch_index** | Option<**String**> | A query parameter denoting the requests watch index. |  |

### Return type

[**crate::models::PartyGetSummaryOutput**](PartyGetSummaryOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_service_period_join

> crate::models::PartyJoinOutput parties_service_period_join(party_join_input)


Joins a party using a given party invite.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**party_join_input** | [**PartyJoinInput**](PartyJoinInput.md) |  | [required] |

### Return type

[**crate::models::PartyJoinOutput**](PartyJoinOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_service_period_kick_member

> parties_service_period_kick_member(identity_id)


Kicks a member from the current identity's current party. Identity must be the party leader.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_id** | **String** | A universally unique identifier. | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_service_period_leave

> parties_service_period_leave()


Leaves the current identity's party.

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


## parties_service_period_revoke_invite

> parties_service_period_revoke_invite(invite_id)


Revokes a party invite from the current identity's party. Identity must be the party leader.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invite_id** | **String** | A universally unique identifier. | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_service_period_send_join_request

> parties_service_period_send_join_request(party_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**party_id** | **String** | A universally unique identifier. | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_service_period_set_publicity

> parties_service_period_set_publicity(party_set_publicity_input)


Sets the publicity of a party. This configures who can view and join the party. Identity must be the party leader.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**party_set_publicity_input** | [**PartySetPublicityInput**](PartySetPublicityInput.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_service_period_transfer_ownership

> parties_service_period_transfer_ownership(identity_id)


Transfers ownership of the party to another party member. Identity must be the party leader.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_id** | **String** | A universally unique identifier. | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

