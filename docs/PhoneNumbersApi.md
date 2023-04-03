# \PhoneNumbersApi

All URIs are relative to *https://api.clerk.dev/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_phone_number**](PhoneNumbersApi.md#create_phone_number) | **POST** /phone_numbers | Create a phone number
[**delete_phone_number**](PhoneNumbersApi.md#delete_phone_number) | **DELETE** /phone_numbers/{phone_number_id} | Delete a phone number
[**get_phone_number**](PhoneNumbersApi.md#get_phone_number) | **GET** /phone_numbers/{phone_number_id} | Retrieve a phone number
[**update_phone_number**](PhoneNumbersApi.md#update_phone_number) | **PATCH** /phone_numbers/{phone_number_id} | Update a phone number



## create_phone_number

> crate::models::PhoneNumber create_phone_number(create_phone_number_request)
Create a phone number

Create a new phone number

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_phone_number_request** | Option<[**CreatePhoneNumberRequest**](CreatePhoneNumberRequest.md)> |  |  |

### Return type

[**crate::models::PhoneNumber**](PhoneNumber.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_phone_number

> crate::models::DeletedObject delete_phone_number(phone_number_id)
Delete a phone number

Delete the phone number with the given ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**phone_number_id** | **String** | The ID of the phone number to delete | [required] |

### Return type

[**crate::models::DeletedObject**](DeletedObject.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_phone_number

> crate::models::PhoneNumber get_phone_number(phone_number_id)
Retrieve a phone number

Returns the details of a phone number

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**phone_number_id** | **String** | The ID of the phone number to retrieve | [required] |

### Return type

[**crate::models::PhoneNumber**](PhoneNumber.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_phone_number

> crate::models::PhoneNumber update_phone_number(phone_number_id, update_phone_number_request)
Update a phone number

Updates a phone number

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**phone_number_id** | **String** | The ID of the phone number to update | [required] |
**update_phone_number_request** | Option<[**UpdatePhoneNumberRequest**](UpdatePhoneNumberRequest.md)> |  |  |

### Return type

[**crate::models::PhoneNumber**](PhoneNumber.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

