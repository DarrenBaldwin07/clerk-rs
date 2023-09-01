# \RedirectUrlsApi

All URIs are relative to *https://api.clerk.dev/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_redirect_url**](RedirectUrlsApi.md#create_redirect_url) | **POST** /redirect_urls | 
[**delete_redirect_url**](RedirectUrlsApi.md#delete_redirect_url) | **DELETE** /redirect_urls/{id} | Delete a redirect URL
[**get_redirect_url**](RedirectUrlsApi.md#get_redirect_url) | **GET** /redirect_urls/{id} | Retrieve a redirect URL
[**list_redirect_urls**](RedirectUrlsApi.md#list_redirect_urls) | **GET** /redirect_urls | List all redirect URLs



## create_redirect_url

> crate::models::RedirectUrl create_redirect_url(create_redirect_url_request)


Create a redirect URL

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_redirect_url_request** | Option<[**CreateRedirectUrlRequest**](CreateRedirectUrlRequest.md)> |  |  |

### Return type

[**crate::models::RedirectUrl**](RedirectURL.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_redirect_url

> crate::models::DeletedObject delete_redirect_url(id)
Delete a redirect URL

Remove the selected redirect URL from the whitelist of the instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the redirect URL | [required] |

### Return type

[**crate::models::DeletedObject**](DeletedObject.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_redirect_url

> crate::models::RedirectUrl get_redirect_url(id)
Retrieve a redirect URL

Retrieve the details of the redirect URL with the given ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the redirect URL | [required] |

### Return type

[**crate::models::RedirectUrl**](RedirectURL.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_redirect_urls

> Vec<crate::models::RedirectUrl> list_redirect_urls()
List all redirect URLs

Lists all whitelisted redirect_urls for the instance

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::RedirectUrl>**](RedirectURL.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

