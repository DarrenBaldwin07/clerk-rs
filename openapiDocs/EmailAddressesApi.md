# \EmailAddressesApi

All URIs are relative to *https://api.clerk.dev/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_email_address**](EmailAddressesApi.md#create_email_address) | **POST** /email_addresses | Create an email address
[**delete_email_address**](EmailAddressesApi.md#delete_email_address) | **DELETE** /email_addresses/{email_address_id} | Delete an email address
[**get_email_address**](EmailAddressesApi.md#get_email_address) | **GET** /email_addresses/{email_address_id} | Retrieve an email address
[**update_email_address**](EmailAddressesApi.md#update_email_address) | **PATCH** /email_addresses/{email_address_id} | Update an email address



## create_email_address

> crate::models::EmailAddress create_email_address(create_email_address_request)
Create an email address

Create a new email address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_email_address_request** | Option<[**CreateEmailAddressRequest**](CreateEmailAddressRequest.md)> |  |  |

### Return type

[**crate::models::EmailAddress**](EmailAddress.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_email_address

> crate::models::DeletedObject delete_email_address(email_address_id)
Delete an email address

Delete the email address with the given ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_address_id** | **String** | The ID of the email address to delete | [required] |

### Return type

[**crate::models::DeletedObject**](DeletedObject.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_email_address

> crate::models::EmailAddress get_email_address(email_address_id)
Retrieve an email address

Returns the details of an email address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_address_id** | **String** | The ID of the email address to retrieve | [required] |

### Return type

[**crate::models::EmailAddress**](EmailAddress.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_email_address

> crate::models::EmailAddress update_email_address(email_address_id, update_email_address_request)
Update an email address

Updates an email address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_address_id** | **String** | The ID of the email address to update | [required] |
**update_email_address_request** | Option<[**UpdateEmailAddressRequest**](UpdateEmailAddressRequest.md)> |  |  |

### Return type

[**crate::models::EmailAddress**](EmailAddress.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

