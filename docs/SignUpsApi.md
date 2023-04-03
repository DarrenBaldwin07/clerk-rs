# \SignUpsApi

All URIs are relative to *https://api.clerk.dev/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**update_sign_up**](SignUpsApi.md#update_sign_up) | **PATCH** /sign_ups/{id} | Update a sign-up



## update_sign_up

> crate::models::SignUp update_sign_up(id, update_sign_up_request)
Update a sign-up

Update the sign-up with the given ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the sign-up to update | [required] |
**update_sign_up_request** | Option<[**UpdateSignUpRequest**](UpdateSignUpRequest.md)> |  |  |

### Return type

[**crate::models::SignUp**](SignUp.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

