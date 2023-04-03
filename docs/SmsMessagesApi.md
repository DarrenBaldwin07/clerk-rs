# \SmsMessagesApi

All URIs are relative to *https://api.clerk.dev/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_sms_message**](SmsMessagesApi.md#create_sms_message) | **POST** /sms_messages | Create an SMS message



## create_sms_message

> crate::models::SmsMessage create_sms_message(create_sms_message_request)
Create an SMS message

Create and send an SMS message to the supplied phone number ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_sms_message_request** | Option<[**CreateSmsMessageRequest**](CreateSmsMessageRequest.md)> | Required parameters |  |

### Return type

[**crate::models::SmsMessage**](SMSMessage.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

