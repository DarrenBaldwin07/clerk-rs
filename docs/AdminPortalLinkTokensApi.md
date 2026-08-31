# \AdminPortalLinkTokensApi

All URIs are relative to *https://api.clerk.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_admin_portal_link_token**](AdminPortalLinkTokensApi.md#create_admin_portal_link_token) | **POST** /admin_portal_link_tokens | Create an Admin Portal Link Token
[**revoke_admin_portal_link_token**](AdminPortalLinkTokensApi.md#revoke_admin_portal_link_token) | **POST** /admin_portal_link_tokens/{adminPortalLinkTokenID}/revoke | Revoke an Admin Portal Link Token



## create_admin_portal_link_token

> models::CreateAdminPortalLinkToken201Response create_admin_portal_link_token(create_admin_portal_link_token_request)
Create an Admin Portal Link Token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_admin_portal_link_token_request** | [**CreateAdminPortalLinkTokenRequest**](CreateAdminPortalLinkTokenRequest.md) |  | [required] |

### Return type

[**models::CreateAdminPortalLinkToken201Response**](createAdminPortalLinkToken_201_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_admin_portal_link_token

> models::RevokeAdminPortalLinkToken200Response revoke_admin_portal_link_token(admin_portal_link_token_id, revoke_admin_portal_link_token_request)
Revoke an Admin Portal Link Token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**admin_portal_link_token_id** | **String** |  | [required] |
**revoke_admin_portal_link_token_request** | [**RevokeAdminPortalLinkTokenRequest**](RevokeAdminPortalLinkTokenRequest.md) |  | [required] |

### Return type

[**models::RevokeAdminPortalLinkToken200Response**](revokeAdminPortalLinkToken_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

