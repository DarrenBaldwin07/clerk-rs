# \WebhooksApi

All URIs are relative to *https://api.clerk.dev/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_svix_app**](WebhooksApi.md#create_svix_app) | **POST** /webhooks/svix | Create a Svix app
[**delete_svix_app**](WebhooksApi.md#delete_svix_app) | **DELETE** /webhooks/svix | Delete a Svix app
[**generate_svix_auth_url**](WebhooksApi.md#generate_svix_auth_url) | **POST** /webhooks/svix_url | Create a Svix Dashboard URL



## create_svix_app

> crate::models::SvixUrl create_svix_app()
Create a Svix app

Create a Svix app and associate it with the current instance

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SvixUrl**](SvixURL.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_svix_app

> delete_svix_app()
Delete a Svix app

Delete a Svix app and disassociate it from the current instance

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_svix_auth_url

> crate::models::SvixUrl generate_svix_auth_url()
Create a Svix Dashboard URL

Generate a new url for accessing the Svix's management dashboard for that particular instance

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SvixUrl**](SvixURL.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

