# \PartyPartiesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**parties_service_period_create_party**](PartyPartiesApi.md#parties_service_period_create_party) | **POST** /parties | 
[**parties_service_period_create_party_invite**](PartyPartiesApi.md#parties_service_period_create_party_invite) | **POST** /parties/self/invites | 
[**parties_service_period_get_party_from_invite**](PartyPartiesApi.md#parties_service_period_get_party_from_invite) | **GET** /invites | 
[**parties_service_period_get_party_profile**](PartyPartiesApi.md#parties_service_period_get_party_profile) | **GET** /parties/{party_id}/profile | 
[**parties_service_period_get_party_self_profile**](PartyPartiesApi.md#parties_service_period_get_party_self_profile) | **GET** /parties/self/profile | 
[**parties_service_period_get_party_self_summary**](PartyPartiesApi.md#parties_service_period_get_party_self_summary) | **GET** /parties/self/summary | 
[**parties_service_period_get_party_summary**](PartyPartiesApi.md#parties_service_period_get_party_summary) | **GET** /parties/{party_id}/summary | 
[**parties_service_period_join_party**](PartyPartiesApi.md#parties_service_period_join_party) | **POST** /parties/join | 
[**parties_service_period_kick_member**](PartyPartiesApi.md#parties_service_period_kick_member) | **POST** /parties/self/members/{identity_id}/kick | 
[**parties_service_period_leave_party**](PartyPartiesApi.md#parties_service_period_leave_party) | **POST** /parties/self/leave | 
[**parties_service_period_revoke_party_invite**](PartyPartiesApi.md#parties_service_period_revoke_party_invite) | **DELETE** /parties/self/invites/{invite_id} | 
[**parties_service_period_send_join_request**](PartyPartiesApi.md#parties_service_period_send_join_request) | **POST** /parties/{party_id}/join-request/send | 
[**parties_service_period_set_party_publicity**](PartyPartiesApi.md#parties_service_period_set_party_publicity) | **PUT** /parties/self/publicity | 
[**parties_service_period_transfer_party_ownership**](PartyPartiesApi.md#parties_service_period_transfer_party_ownership) | **POST** /parties/self/members/{identity_id}/transfer-ownership | 



## parties_service_period_create_party

> crate::models::PartyCreatePartyOutput parties_service_period_create_party(party_create_party_input)


Creates a new party.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**party_create_party_input** | [**PartyCreatePartyInput**](PartyCreatePartyInput.md) |  | [required] |

### Return type

[**crate::models::PartyCreatePartyOutput**](PartyCreatePartyOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_service_period_create_party_invite

> crate::models::PartyCreatePartyInviteOutput parties_service_period_create_party_invite(party_create_party_invite_input)


Creates a new party invite for the current identity's party. Identity must be the party leader.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**party_create_party_invite_input** | [**PartyCreatePartyInviteInput**](PartyCreatePartyInviteInput.md) |  | [required] |

### Return type

[**crate::models::PartyCreatePartyInviteOutput**](PartyCreatePartyInviteOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_service_period_get_party_from_invite

> crate::models::PartyGetPartyFromInviteOutput parties_service_period_get_party_from_invite(token, alias)


Fetches a party based on a given invite.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | See `rivet.api.party#CreatedInvite$token`. |  |
**alias** | Option<**String**> | An alias used to join a given party. This alias must be unique for all invites for your game. Pass this alias to `rivet.api.party.common#CreatedInvite$alias` to consume the invite. |  |

### Return type

[**crate::models::PartyGetPartyFromInviteOutput**](PartyGetPartyFromInviteOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_service_period_get_party_profile

> crate::models::PartyGetPartyProfileOutput parties_service_period_get_party_profile(party_id, watch_index)


Returns a party profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**party_id** | **String** | A universally unique identifier. | [required] |
**watch_index** | Option<**String**> | A query parameter denoting the requests watch index. |  |

### Return type

[**crate::models::PartyGetPartyProfileOutput**](PartyGetPartyProfileOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_service_period_get_party_self_profile

> crate::models::PartyGetPartySelfProfileOutput parties_service_period_get_party_self_profile(watch_index)


Returns a party profile for the party the current identity is a member of.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**watch_index** | Option<**String**> | A query parameter denoting the requests watch index. |  |

### Return type

[**crate::models::PartyGetPartySelfProfileOutput**](PartyGetPartySelfProfileOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_service_period_get_party_self_summary

> crate::models::PartyGetPartySelfSummaryOutput parties_service_period_get_party_self_summary(watch_index)


Returns a party summary for the party the current identity is a member of.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**watch_index** | Option<**String**> | A query parameter denoting the requests watch index. |  |

### Return type

[**crate::models::PartyGetPartySelfSummaryOutput**](PartyGetPartySelfSummaryOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_service_period_get_party_summary

> crate::models::PartyGetPartySummaryOutput parties_service_period_get_party_summary(party_id, watch_index)


Returns a party summary.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**party_id** | **String** | A universally unique identifier. | [required] |
**watch_index** | Option<**String**> | A query parameter denoting the requests watch index. |  |

### Return type

[**crate::models::PartyGetPartySummaryOutput**](PartyGetPartySummaryOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_service_period_join_party

> crate::models::PartyJoinPartyOutput parties_service_period_join_party(party_join_party_input)


Joins a party using a given party invite.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**party_join_party_input** | [**PartyJoinPartyInput**](PartyJoinPartyInput.md) |  | [required] |

### Return type

[**crate::models::PartyJoinPartyOutput**](PartyJoinPartyOutput.md)

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


## parties_service_period_leave_party

> parties_service_period_leave_party()


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


## parties_service_period_revoke_party_invite

> parties_service_period_revoke_party_invite(invite_id)


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


## parties_service_period_set_party_publicity

> parties_service_period_set_party_publicity(party_set_party_publicity_input)


Sets the publicity of a party. This configures who can view and join the party. Identity must be the party leader.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**party_set_party_publicity_input** | [**PartySetPartyPublicityInput**](PartySetPartyPublicityInput.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_service_period_transfer_party_ownership

> parties_service_period_transfer_party_ownership(identity_id)


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

