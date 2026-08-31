# \PhoneNumbersApi

All URIs are relative to *https://api.clerk.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**attempt_phone_number_verification**](PhoneNumbersApi.md#attempt_phone_number_verification) | **POST** /phone_numbers/{phone_number_id}/attempt_verification | Verify a code sent to a phone number
[**create_phone_number**](PhoneNumbersApi.md#create_phone_number) | **POST** /phone_numbers | Create a phone number
[**delete_phone_number**](PhoneNumbersApi.md#delete_phone_number) | **DELETE** /phone_numbers/{phone_number_id} | Delete a phone number
[**get_phone_number**](PhoneNumbersApi.md#get_phone_number) | **GET** /phone_numbers/{phone_number_id} | Retrieve a phone number
[**prepare_phone_number_verification**](PhoneNumbersApi.md#prepare_phone_number_verification) | **POST** /phone_numbers/{phone_number_id}/prepare_verification | Send a verification code to a phone number
[**replace_user_phone_number**](PhoneNumbersApi.md#replace_user_phone_number) | **PUT** /users/{user_id}/phone_number | Replace a user's phone number
[**update_phone_number**](PhoneNumbersApi.md#update_phone_number) | **PATCH** /phone_numbers/{phone_number_id} | Update a phone number



## attempt_phone_number_verification

> models::PrepareEmailAddressVerification200Response attempt_phone_number_verification(phone_number_id, attempt_phone_number_verification_request)
Verify a code sent to a phone number

Checks a one-time code against the verification identified by verification_id, and returns the verification with its updated status (`verified`, `unverified`, `expired`, or `failed`) and attempt count, so a backend driving its own frontend can react on every attempt — an incorrect or expired code is reported through the status, not as an error. Resubmitting a verification whose code was already accepted is rejected with a `verification_already_verified` error. If the code is correct and the phone number is not already verified, it is also marked as verified as a side effect (just as it would be in a frontend verification flow); an already verified phone number is left unchanged. It never creates a session; to sign the user in afterwards, mint a sign-in token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**phone_number_id** | **String** | The ID of the phone number whose code is being verified | [required] |
**attempt_phone_number_verification_request** | [**AttemptPhoneNumberVerificationRequest**](AttemptPhoneNumberVerificationRequest.md) |  | [required] |

### Return type

[**models::PrepareEmailAddressVerification200Response**](PrepareEmailAddressVerification_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_phone_number

> models::PhoneNumber create_phone_number(create_phone_number_request)
Create a phone number

Create a new phone number

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_phone_number_request** | Option<[**CreatePhoneNumberRequest**](CreatePhoneNumberRequest.md)> |  |  |

### Return type

[**models::PhoneNumber**](PhoneNumber.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_phone_number

> models::DeletedObject delete_phone_number(phone_number_id)
Delete a phone number

Delete the phone number with the given ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**phone_number_id** | **String** | The ID of the phone number to delete | [required] |

### Return type

[**models::DeletedObject**](DeletedObject.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_phone_number

> models::PhoneNumber get_phone_number(phone_number_id)
Retrieve a phone number

Returns the details of a phone number

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**phone_number_id** | **String** | The ID of the phone number to retrieve | [required] |

### Return type

[**models::PhoneNumber**](PhoneNumber.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prepare_phone_number_verification

> models::PrepareEmailAddressVerification200Response prepare_phone_number_verification(phone_number_id)
Send a verification code to a phone number

Sends a one-time code to the given phone number so that a backend can verify the user controls it (for example, in a custom, backend-driven sign-in flow). The code is tracked on its own verification; confirm it with attempt_verification.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**phone_number_id** | **String** | The ID of the phone number to send the verification code to | [required] |

### Return type

[**models::PrepareEmailAddressVerification200Response**](PrepareEmailAddressVerification_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_user_phone_number

> models::PhoneNumber replace_user_phone_number(user_id, replace_user_phone_number_request)
Replace a user's phone number

Replaces all of the user's phone numbers with a single primary phone number. By default the new phone number is created verified, with the admin verification strategy. When `identification_status` is `reserved` it is created reserved instead: unverified but usable for sign-in and locked so no other user can claim it. When it is `unverified` the phone number is neither usable for sign-in nor locked. The new phone number is never reserved for second factor. Any existing phone numbers are deleted; replacing a phone number that is reserved for second factor disables the user's MFA.  **Warning:** `identification_status: unverified` can lock the user out of their account. An unverified phone number cannot be used to sign in, so if the user has no other verified or reserved identifier, deleting their existing phone numbers leaves them unable to authenticate — and unable to verify the new number, since that requires signing in. Recovery then requires another admin API call.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user whose phone number to replace | [required] |
**replace_user_phone_number_request** | [**ReplaceUserPhoneNumberRequest**](ReplaceUserPhoneNumberRequest.md) |  | [required] |

### Return type

[**models::PhoneNumber**](PhoneNumber.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_phone_number

> models::PhoneNumber update_phone_number(phone_number_id, update_phone_number_request)
Update a phone number

Updates a phone number

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**phone_number_id** | **String** | The ID of the phone number to update | [required] |
**update_phone_number_request** | Option<[**UpdatePhoneNumberRequest**](UpdatePhoneNumberRequest.md)> |  |  |

### Return type

[**models::PhoneNumber**](PhoneNumber.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

