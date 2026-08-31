# \MiscellaneousApi

All URIs are relative to *https://api.clerk.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_public_interstitial**](MiscellaneousApi.md#get_public_interstitial) | **GET** /public/interstitial | Returns the markup for the interstitial page



## get_public_interstitial

> get_public_interstitial(frontend_api, frontend_api2, publishable_key, proxy_url, domain, sign_in_url, use_domain_for_script)
Returns the markup for the interstitial page

The Clerk interstitial endpoint serves an html page that loads clerk.js in order to check the user's authentication state. It is used by Clerk SDKs when the user's authentication state cannot be immediately determined.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**frontend_api** | Option<**String**> | Please use `frontend_api` instead |  |
**frontend_api2** | Option<**String**> | The Frontend API key of your instance |  |
**publishable_key** | Option<**String**> | The publishable key of your instance |  |
**proxy_url** | Option<**String**> | The proxy URL of your instance |  |
**domain** | Option<**String**> | The domain of your instance |  |
**sign_in_url** | Option<**String**> | The sign in URL of your instance |  |
**use_domain_for_script** | Option<**bool**> | Whether to use the domain for the script URL |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

