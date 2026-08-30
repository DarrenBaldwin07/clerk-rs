# \ApiKeysApi

All URIs are relative to *https://api.clerk.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_api_key**](ApiKeysApi.md#create_api_key) | **POST** /api_keys | Create an API Key
[**delete_api_key**](ApiKeysApi.md#delete_api_key) | **DELETE** /api_keys/{apiKeyID} | Delete an API Key
[**get_api_key**](ApiKeysApi.md#get_api_key) | **GET** /api_keys/{apiKeyID} | Get an API Key by ID
[**get_api_key_secret**](ApiKeysApi.md#get_api_key_secret) | **GET** /api_keys/{apiKeyID}/secret | Get an API Key Secret
[**get_api_keys**](ApiKeysApi.md#get_api_keys) | **GET** /api_keys | Get API Keys
[**revoke_api_key**](ApiKeysApi.md#revoke_api_key) | **POST** /api_keys/{apiKeyID}/revoke | Revoke an API Key
[**update_api_key**](ApiKeysApi.md#update_api_key) | **PATCH** /api_keys/{apiKeyID} | Update an API Key
[**verify_api_key**](ApiKeysApi.md#verify_api_key) | **POST** /api_keys/verify | Verify an API Key



## create_api_key

> models::CreateApiKey200Response create_api_key(create_api_key_request)
Create an API Key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_api_key_request** | [**CreateApiKeyRequest**](CreateApiKeyRequest.md) |  | [required] |

### Return type

[**models::CreateApiKey200Response**](createApiKey_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_key

> models::DeleteApiKey200Response delete_api_key(api_key_id)
Delete an API Key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key_id** | **String** |  | [required] |

### Return type

[**models::DeleteApiKey200Response**](deleteApiKey_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_key

> models::GetApiKeys200ResponseDataInner get_api_key(api_key_id)
Get an API Key by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key_id** | **String** |  | [required] |

### Return type

[**models::GetApiKeys200ResponseDataInner**](getApiKeys_200_response_data_inner.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_key_secret

> models::GetApiKeySecret200Response get_api_key_secret(api_key_id)
Get an API Key Secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key_id** | **String** |  | [required] |

### Return type

[**models::GetApiKeySecret200Response**](getApiKeySecret_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_keys

> models::GetApiKeys200Response get_api_keys(subject, r#type, include_invalid, limit, offset, query)
Get API Keys

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject** | **String** |  | [required] |
**r#type** | Option<**String**> |  |  |[default to api_key]
**include_invalid** | Option<**String**> |  |  |[default to false]
**limit** | Option<**f64**> |  |  |[default to 10]
**offset** | Option<**f64**> |  |  |[default to 0]
**query** | Option<**String**> |  |  |

### Return type

[**models::GetApiKeys200Response**](getApiKeys_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_api_key

> models::GetApiKeys200ResponseDataInner revoke_api_key(api_key_id, revoke_admin_portal_link_token_request)
Revoke an API Key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key_id** | **String** |  | [required] |
**revoke_admin_portal_link_token_request** | [**RevokeAdminPortalLinkTokenRequest**](RevokeAdminPortalLinkTokenRequest.md) |  | [required] |

### Return type

[**models::GetApiKeys200ResponseDataInner**](getApiKeys_200_response_data_inner.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_api_key

> models::GetApiKeys200ResponseDataInner update_api_key(api_key_id, update_api_key_request)
Update an API Key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key_id** | **String** |  | [required] |
**update_api_key_request** | [**UpdateApiKeyRequest**](UpdateApiKeyRequest.md) |  | [required] |

### Return type

[**models::GetApiKeys200ResponseDataInner**](getApiKeys_200_response_data_inner.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_api_key

> models::GetApiKeys200ResponseDataInner verify_api_key(verify_api_key_request)
Verify an API Key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verify_api_key_request** | [**VerifyApiKeyRequest**](VerifyApiKeyRequest.md) |  | [required] |

### Return type

[**models::GetApiKeys200ResponseDataInner**](getApiKeys_200_response_data_inner.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

