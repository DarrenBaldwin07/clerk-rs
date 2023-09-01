# \ActorTokensApi

All URIs are relative to *https://api.clerk.dev/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_actor_token**](ActorTokensApi.md#create_actor_token) | **POST** /actor_tokens | Create actor token
[**revoke_actor_token**](ActorTokensApi.md#revoke_actor_token) | **POST** /actor_tokens/{actor_token_id}/revoke | Revoke actor token



## create_actor_token

> crate::models::ActorToken create_actor_token(create_actor_token_request)
Create actor token

Create an actor token that can be used to impersonate the given user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_actor_token_request** | Option<[**CreateActorTokenRequest**](CreateActorTokenRequest.md)> |  |  |

### Return type

[**crate::models::ActorToken**](ActorToken.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_actor_token

> crate::models::ActorToken revoke_actor_token(actor_token_id)
Revoke actor token

Revokes a pending actor token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**actor_token_id** | **String** | The ID of the actor token to be revoked. | [required] |

### Return type

[**crate::models::ActorToken**](ActorToken.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

