# \MiscellaneousApi

All URIs are relative to *https://api.clerk.dev/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_demo_instance**](MiscellaneousApi.md#create_demo_instance) | **POST** /public/demo_instance | Create a demo development instance
[**get_public_interstitial**](MiscellaneousApi.md#get_public_interstitial) | **GET** /public/interstitial | Returns the markup for the interstitial page



## create_demo_instance

> crate::models::CreateDemoInstance200Response create_demo_instance()
Create a demo development instance

Creates a demo development instance and returns the corresponding Frontend/Backend API keys.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateDemoInstance200Response**](CreateDemoInstance_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_public_interstitial

> get_public_interstitial(frontend_api, publishable_key)
Returns the markup for the interstitial page

The Clerk interstitial endpoint serves an html page that loads clerk.js in order to check the user's authentication state. It is used by Clerk SDKs when the user's authentication state cannot be immediately determined.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**frontend_api** | Option<**String**> | The Frontend API key of your instance |  |
**publishable_key** | Option<**String**> | The publishable key of your instance |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

