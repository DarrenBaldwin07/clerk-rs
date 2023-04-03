# \SignInTokensApi

All URIs are relative to *https://api.clerk.dev/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_sign_in_token**](SignInTokensApi.md#create_sign_in_token) | **POST** /sign_in_tokens | Create sign-in token
[**revoke_sign_in_token**](SignInTokensApi.md#revoke_sign_in_token) | **POST** /sign_in_tokens/{sign_in_token_id}/revoke | Revoke the given sign-in token



## create_sign_in_token

> crate::models::SignInToken create_sign_in_token(create_sign_in_token_request)
Create sign-in token

Creates a new sign-in token and associates it with the given user. By default, sign-in tokens expire in 30 days. You can optionally supply a different duration in seconds using the `expires_in_seconds` property.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_sign_in_token_request** | Option<[**CreateSignInTokenRequest**](CreateSignInTokenRequest.md)> |  |  |

### Return type

[**crate::models::SignInToken**](SignInToken.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_sign_in_token

> crate::models::SignInToken revoke_sign_in_token(sign_in_token_id)
Revoke the given sign-in token

Revokes a pending sign-in token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sign_in_token_id** | **String** | The ID of the sign-in token to be revoked | [required] |

### Return type

[**crate::models::SignInToken**](SignInToken.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

