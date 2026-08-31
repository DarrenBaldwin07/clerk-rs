# \OAuthAccessTokensApi

All URIs are relative to *https://api.clerk.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**verify_o_auth_access_token**](OAuthAccessTokensApi.md#verify_o_auth_access_token) | **POST** /oauth_applications/access_tokens/verify | Verify an OAuth Access Token



## verify_o_auth_access_token

> models::VerifyOAuthAccessToken200Response verify_o_auth_access_token(verify_o_auth_access_token_request)
Verify an OAuth Access Token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verify_o_auth_access_token_request** | [**VerifyOAuthAccessTokenRequest**](VerifyOAuthAccessTokenRequest.md) |  | [required] |

### Return type

[**models::VerifyOAuthAccessToken200Response**](verifyOAuthAccessToken_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

