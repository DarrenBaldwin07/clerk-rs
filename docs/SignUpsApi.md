# \SignUpsApi

All URIs are relative to *https://api.clerk.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_sign_up**](SignUpsApi.md#get_sign_up) | **GET** /sign_ups/{id} | Retrieve a sign-up by ID
[**update_sign_up**](SignUpsApi.md#update_sign_up) | **PATCH** /sign_ups/{id} | Update a sign-up



## get_sign_up

> models::SignUp get_sign_up(id)
Retrieve a sign-up by ID

Retrieve the details of the sign-up with the given ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the sign-up to retrieve | [required] |

### Return type

[**models::SignUp**](SignUp.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_sign_up

> models::SignUp update_sign_up(id, update_sign_up_request)
Update a sign-up

Update the sign-up with the given ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the sign-up to update | [required] |
**update_sign_up_request** | Option<[**UpdateSignUpRequest**](UpdateSignUpRequest.md)> |  |  |

### Return type

[**models::SignUp**](SignUp.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

