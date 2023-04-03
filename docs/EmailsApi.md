# \EmailsApi

All URIs are relative to *https://api.clerk.dev/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_email**](EmailsApi.md#create_email) | **POST** /emails | Create an email



## create_email

> crate::models::Email create_email(create_email_request)
Create an email

Create and send an email to the supplied email address ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_email_request** | Option<[**CreateEmailRequest**](CreateEmailRequest.md)> | Required parameters |  |

### Return type

[**crate::models::Email**](Email.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

