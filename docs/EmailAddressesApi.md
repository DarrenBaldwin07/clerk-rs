# \EmailAddressesApi

All URIs are relative to *https://api.clerk.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**attempt_email_address_verification**](EmailAddressesApi.md#attempt_email_address_verification) | **POST** /email_addresses/{email_address_id}/attempt_verification | Verify a code sent to an email address
[**create_email_address**](EmailAddressesApi.md#create_email_address) | **POST** /email_addresses | Create an email address
[**delete_email_address**](EmailAddressesApi.md#delete_email_address) | **DELETE** /email_addresses/{email_address_id} | Delete an email address
[**get_email_address**](EmailAddressesApi.md#get_email_address) | **GET** /email_addresses/{email_address_id} | Retrieve an email address
[**prepare_email_address_verification**](EmailAddressesApi.md#prepare_email_address_verification) | **POST** /email_addresses/{email_address_id}/prepare_verification | Send a verification code to an email address
[**replace_user_email_address**](EmailAddressesApi.md#replace_user_email_address) | **PUT** /users/{user_id}/email_address | Replace a user's email address
[**update_email_address**](EmailAddressesApi.md#update_email_address) | **PATCH** /email_addresses/{email_address_id} | Update an email address



## attempt_email_address_verification

> models::PrepareEmailAddressVerification200Response attempt_email_address_verification(email_address_id, attempt_email_address_verification_request)
Verify a code sent to an email address

Checks a one-time code against the verification identified by verification_id, and returns the verification with its updated status (`verified`, `unverified`, `expired`, or `failed`) and attempt count, so a backend driving its own frontend can react on every attempt — an incorrect or expired code is reported through the status, not as an error. Resubmitting a verification whose code was already accepted is rejected with a `verification_already_verified` error. If the code is correct and the email address is not already verified, it is also marked as verified as a side effect (just as it would be in a frontend verification flow); an already verified email address is left unchanged. It never creates a session; to sign the user in afterwards, mint a sign-in token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_address_id** | **String** | The ID of the email address whose code is being verified | [required] |
**attempt_email_address_verification_request** | [**AttemptEmailAddressVerificationRequest**](AttemptEmailAddressVerificationRequest.md) |  | [required] |

### Return type

[**models::PrepareEmailAddressVerification200Response**](PrepareEmailAddressVerification_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_email_address

> models::EmailAddress create_email_address(create_email_address_request)
Create an email address

Create a new email address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_email_address_request** | Option<[**CreateEmailAddressRequest**](CreateEmailAddressRequest.md)> |  |  |

### Return type

[**models::EmailAddress**](EmailAddress.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_email_address

> models::DeletedObject delete_email_address(email_address_id)
Delete an email address

Delete the email address with the given ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_address_id** | **String** | The ID of the email address to delete | [required] |

### Return type

[**models::DeletedObject**](DeletedObject.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_email_address

> models::EmailAddress get_email_address(email_address_id)
Retrieve an email address

Returns the details of an email address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_address_id** | **String** | The ID of the email address to retrieve | [required] |

### Return type

[**models::EmailAddress**](EmailAddress.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prepare_email_address_verification

> models::PrepareEmailAddressVerification200Response prepare_email_address_verification(email_address_id)
Send a verification code to an email address

Sends a one-time code to the given email address so that a backend can verify the user controls it (for example, in a custom, backend-driven sign-in flow). The code is tracked on its own verification; confirm it with attempt_verification.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_address_id** | **String** | The ID of the email address to send the verification code to | [required] |

### Return type

[**models::PrepareEmailAddressVerification200Response**](PrepareEmailAddressVerification_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_user_email_address

> models::EmailAddress replace_user_email_address(user_id, replace_user_email_address_request)
Replace a user's email address

Replaces all of the user's email addresses with a single primary email address. By default the new email address is created verified, with the admin verification strategy. When `identification_status` is `reserved` it is created reserved instead: unverified but usable for sign-in and locked so no other user can claim it. When it is `unverified` the address is neither usable for sign-in nor locked. Any existing email addresses are deleted. If an existing email address is linked to a connected account, the request is rejected; remove the connected account first.  **Warning:** `identification_status: unverified` can lock the user out of their account. An unverified email address cannot be used to sign in, so if the user has no other verified or reserved identifier, deleting their existing email addresses leaves them unable to authenticate — and unable to verify the new address, since that requires signing in. Recovery then requires another admin API call.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user whose email address to replace | [required] |
**replace_user_email_address_request** | [**ReplaceUserEmailAddressRequest**](ReplaceUserEmailAddressRequest.md) |  | [required] |

### Return type

[**models::EmailAddress**](EmailAddress.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_email_address

> models::EmailAddress update_email_address(email_address_id, update_email_address_request)
Update an email address

Updates an email address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_address_id** | **String** | The ID of the email address to update | [required] |
**update_email_address_request** | Option<[**UpdateEmailAddressRequest**](UpdateEmailAddressRequest.md)> |  |  |

### Return type

[**models::EmailAddress**](EmailAddress.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

