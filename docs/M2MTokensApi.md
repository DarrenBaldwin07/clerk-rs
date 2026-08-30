# \M2MTokensApi

All URIs are relative to *https://api.clerk.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_m2_m_token**](M2MTokensApi.md#create_m2_m_token) | **POST** /m2m_tokens | Create a M2M Token
[**get_m2_m_tokens**](M2MTokensApi.md#get_m2_m_tokens) | **GET** /m2m_tokens | Get M2M Tokens
[**revoke_m2_m_token**](M2MTokensApi.md#revoke_m2_m_token) | **POST** /m2m_tokens/{m2m_token_id}/revoke | Revoke a M2M Token
[**verify_m2_m_token**](M2MTokensApi.md#verify_m2_m_token) | **POST** /m2m_tokens/verify | Verify a M2M Token



## create_m2_m_token

> models::CreateM2MToken201Response create_m2_m_token(create_m2_m_token_request)
Create a M2M Token

Creates a new M2M Token. Must be authenticated via a Machine Secret Key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_m2_m_token_request** | [**CreateM2MTokenRequest**](CreateM2MTokenRequest.md) |  | [required] |

### Return type

[**models::CreateM2MToken201Response**](createM2MToken_201_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_m2_m_tokens

> models::GetM2MTokens200Response get_m2_m_tokens(subject, revoked, expired, limit, offset)
Get M2M Tokens

Fetches M2M tokens for a specific machine.  Only tokens created with the opaque token format are returned by this endpoint. JWT-format M2M tokens are stateless and are not stored.  This endpoint can be authenticated by either a Machine Secret Key or by a Clerk Secret Key.  - When fetching M2M tokens with a Machine Secret Key, only tokens associated with the authenticated machine can be retrieved. - When fetching M2M tokens with a Clerk Secret Key, tokens for any machine in the instance can be retrieved.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject** | **String** |  | [required] |
**revoked** | Option<**bool**> |  |  |[default to false]
**expired** | Option<**bool**> |  |  |[default to false]
**limit** | Option<**f64**> |  |  |[default to 10]
**offset** | Option<**f64**> |  |  |[default to 0]

### Return type

[**models::GetM2MTokens200Response**](getM2MTokens_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_m2_m_token

> models::GetM2MTokens200ResponseM2mTokensInner revoke_m2_m_token(m2m_token_id, revoke_admin_portal_link_token_request)
Revoke a M2M Token

Revokes a M2M Token.  This endpoint only revokes stored opaque-format M2M tokens. JWT-format M2M tokens are stateless and cannot be revoked.  This endpoint can be authenticated by either a Machine Secret Key or by a Clerk Secret Key.  - When revoking a M2M Token with a Machine Secret Key, the token must managed by the Machine associated with the Machine Secret Key. - When revoking a M2M Token with a Clerk Secret Key, any token on the Instance can be revoked.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**m2m_token_id** | **String** |  | [required] |
**revoke_admin_portal_link_token_request** | [**RevokeAdminPortalLinkTokenRequest**](RevokeAdminPortalLinkTokenRequest.md) |  | [required] |

### Return type

[**models::GetM2MTokens200ResponseM2mTokensInner**](getM2MTokens_200_response_m2m_tokens_inner.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_m2_m_token

> models::GetM2MTokens200ResponseM2mTokensInner verify_m2_m_token(verify_m2_m_token_request)
Verify a M2M Token

Verifies a M2M Token.  This endpoint can be authenticated by either a Machine Secret Key or by a Clerk Secret Key.  - When verifying a M2M Token with a Machine Secret Key, the token must be granted access to the Machine associated with the Machine Secret Key. - When verifying a M2M Token with a Clerk Secret Key, any token on the Instance can be verified.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verify_m2_m_token_request** | [**VerifyM2MTokenRequest**](VerifyM2MTokenRequest.md) |  | [required] |

### Return type

[**models::GetM2MTokens200ResponseM2mTokensInner**](getM2MTokens_200_response_m2m_tokens_inner.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

