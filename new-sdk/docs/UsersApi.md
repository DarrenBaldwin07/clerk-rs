# \UsersApi

All URIs are relative to *https://api.clerk.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ban_user**](UsersApi.md#ban_user) | **POST** /users/{user_id}/ban | Ban a user
[**create_user**](UsersApi.md#create_user) | **POST** /users | Create a new user
[**delete_user**](UsersApi.md#delete_user) | **DELETE** /users/{user_id} | Delete a user
[**disable_mfa**](UsersApi.md#disable_mfa) | **DELETE** /users/{user_id}/mfa | Disable a user's MFA methods
[**get_o_auth_access_token**](UsersApi.md#get_o_auth_access_token) | **GET** /users/{user_id}/oauth_access_tokens/{provider} | Retrieve the OAuth access token of a user
[**get_user**](UsersApi.md#get_user) | **GET** /users/{user_id} | Retrieve a user
[**get_user_list**](UsersApi.md#get_user_list) | **GET** /users | List all users
[**get_users_count**](UsersApi.md#get_users_count) | **GET** /users/count | Count users
[**set_user_profile_image**](UsersApi.md#set_user_profile_image) | **POST** /users/{user_id}/profile_image | Set user profile image
[**unban_user**](UsersApi.md#unban_user) | **POST** /users/{user_id}/unban | Unban a user
[**update_user**](UsersApi.md#update_user) | **PATCH** /users/{user_id} | Update a user
[**update_user_metadata**](UsersApi.md#update_user_metadata) | **PATCH** /users/{user_id}/metadata | Merge and update a user's metadata
[**users_get_organization_memberships**](UsersApi.md#users_get_organization_memberships) | **GET** /users/{user_id}/organization_memberships | Retrieve all memberships for a user
[**verify_password**](UsersApi.md#verify_password) | **POST** /users/{user_id}/verify_password | Verify the password of a user
[**verify_totp**](UsersApi.md#verify_totp) | **POST** /users/{user_id}/verify_totp | Verify a TOTP or backup code for a user



## ban_user

> crate::models::User ban_user(user_id)
Ban a user

Marks the given user as banned, which means that all their sessions are revoked and they are not allowed to sign in again.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user to ban | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user

> crate::models::User create_user(create_user_request)
Create a new user

Creates a new user. Your user management settings determine how you should setup your user model.  Any email address and phone number created using this method will be marked as verified.  Note: If you are performing a migration, check out our guide on [zero downtime migrations](https://clerk.com/docs/deployments/import-users).  A rate limit rule of 20 requests per 10 seconds is applied to this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_user_request** | [**CreateUserRequest**](CreateUserRequest.md) |  | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user

> crate::models::DeletedObject delete_user(user_id)
Delete a user

Delete the specified user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user to delete | [required] |

### Return type

[**crate::models::DeletedObject**](DeletedObject.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_mfa

> crate::models::DisableMfa200Response disable_mfa(user_id)
Disable a user's MFA methods

Disable all of a user's MFA methods (e.g. OTP sent via SMS, TOTP on their authenticator app) at once.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user whose MFA methods are to be disabled | [required] |

### Return type

[**crate::models::DisableMfa200Response**](DisableMFA_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_o_auth_access_token

> Vec<crate::models::GetOAuthAccessToken200ResponseInner> get_o_auth_access_token(user_id, provider)
Retrieve the OAuth access token of a user

Fetch the corresponding OAuth access token for a user that has previously authenticated with a particular OAuth provider. For OAuth 2.0, if the access token has expired and we have a corresponding refresh token, the access token will be refreshed transparently the new one will be returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user for which to retrieve the OAuth access token | [required] |
**provider** | **String** | The ID of the OAuth provider (e.g. `oauth_google`) | [required] |

### Return type

[**Vec<crate::models::GetOAuthAccessToken200ResponseInner>**](GetOAuthAccessToken_200_response_inner.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user

> crate::models::User get_user(user_id)
Retrieve a user

Retrieve the details of a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user to retrieve | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_list

> Vec<crate::models::User> get_user_list(email_address, phone_number, external_id, username, web3_wallet, user_id, organization_id, query, limit, offset, order_by)
List all users

Returns a list of all users. The users are returned sorted by creation date, with the newest users appearing first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_address** | Option<[**Vec<String>**](String.md)> | Returns users with the specified email addresses. Accepts up to 100 email addresses. Any email addresses not found are ignored. |  |
**phone_number** | Option<[**Vec<String>**](String.md)> | Returns users with the specified phone numbers. Accepts up to 100 phone numbers. Any phone numbers not found are ignored. |  |
**external_id** | Option<[**Vec<String>**](String.md)> | Returns users with the specified external ids. For each external id, the `+` and `-` can be prepended to the id, which denote whether the respective external id should be included or excluded from the result set. Accepts up to 100 external ids. Any external ids not found are ignored. |  |
**username** | Option<[**Vec<String>**](String.md)> | Returns users with the specified usernames. Accepts up to 100 usernames. Any usernames not found are ignored. |  |
**web3_wallet** | Option<[**Vec<String>**](String.md)> | Returns users with the specified web3 wallet addresses. Accepts up to 100 web3 wallet addresses. Any web3 wallet addressed not found are ignored. |  |
**user_id** | Option<[**Vec<String>**](String.md)> | Returns users with the user ids specified. For each user id, the `+` and `-` can be prepended to the id, which denote whether the respective user id should be included or excluded from the result set. Accepts up to 100 user ids. Any user ids not found are ignored. |  |
**organization_id** | Option<[**Vec<String>**](String.md)> | Returns users that have memberships to the given organizations. For each organization id, the `+` and `-` can be prepended to the id, which denote whether the respective organization should be included or excluded from the result set. Accepts up to 100 organization ids. |  |
**query** | Option<**String**> | Returns users that match the given query. For possible matches, we check the email addresses, phone numbers, usernames, web3 wallets, user ids, first and last names. The query value doesn't need to match the exact value you are looking for, it is capable of partial matches as well. |  |
**limit** | Option<**f32**> | Applies a limit to the number of results returned. Can be used for paginating the results together with `offset`. Must be an integer greater than zero and less than 500. By default, if not supplied, a limit of 10 is used. |  |[default to 10]
**offset** | Option<**f32**> | Skip the first `offset` results when paginating. Needs to be an integer greater or equal to zero. To be used in conjunction with `limit`. |  |[default to 0]
**order_by** | Option<**String**> | Allows to return users in a particular order. At the moment, you can order the returned users either by their `created_at`,`updated_at`,`email_address`,`web3wallet`,`first_name`,`last_name`,`phone_number`,`username`. In order to specify the direction, you can use the `+/-` symbols prepended in the property to order by. For example, if you want users to be returned in descending order according to their `created_at` property, you can use `-created_at`. If you don't use `+` or `-`, then `+` is implied.We only support one `order_by` parameter, and if multiple `order_by` parameters are provided, we will only keep the first one. For example, if you pass `order_by=username&order_by=created_at`, we will consider only the first `order_by` parameter, which is `username`. The `created_at` parameter will be ignored in this case. |  |[default to -created_at]

### Return type

[**Vec<crate::models::User>**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_count

> crate::models::TotalCount get_users_count(email_address, phone_number, external_id, username, web3_wallet, user_id, query)
Count users

Returns a total count of all users that match the given filtering criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_address** | Option<[**Vec<String>**](String.md)> | Counts users with the specified email addresses. Accepts up to 100 email addresses. Any email addresses not found are ignored. |  |
**phone_number** | Option<[**Vec<String>**](String.md)> | Counts users with the specified phone numbers. Accepts up to 100 phone numbers. Any phone numbers not found are ignored. |  |
**external_id** | Option<[**Vec<String>**](String.md)> | Counts users with the specified external ids. Accepts up to 100 external ids. Any external ids not found are ignored. |  |
**username** | Option<[**Vec<String>**](String.md)> | Counts users with the specified usernames. Accepts up to 100 usernames. Any usernames not found are ignored. |  |
**web3_wallet** | Option<[**Vec<String>**](String.md)> | Counts users with the specified web3 wallet addresses. Accepts up to 100 web3 wallet addresses. Any web3 wallet addressed not found are ignored. |  |
**user_id** | Option<[**Vec<String>**](String.md)> | Counts users with the user ids specified. Accepts up to 100 user ids. Any user ids not found are ignored. |  |
**query** | Option<**String**> | Counts users that match the given query. For possible matches, we check the email addresses, phone numbers, usernames, web3 wallets, user ids, first and last names. The query value doesn't need to match the exact value you are looking for, it is capable of partial matches as well. |  |

### Return type

[**crate::models::TotalCount**](TotalCount.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_user_profile_image

> crate::models::User set_user_profile_image(user_id, file)
Set user profile image

Update a user's profile image

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user to update the profile image for | [required] |
**file** | Option<**std::path::PathBuf**> |  |  |

### Return type

[**crate::models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unban_user

> crate::models::User unban_user(user_id)
Unban a user

Removes the ban mark from the given user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user to unban | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> crate::models::User update_user(user_id, update_user_request)
Update a user

Update a user's attributes.  You can set the user's primary contact identifiers (email address and phone numbers) by updating the `primary_email_address_id` and `primary_phone_number_id` attributes respectively. Both IDs should correspond to verified identifications that belong to the user.  You can remove a user's username by setting the username attribute to null or the blank string \"\". This is a destructive action; the identification will be deleted forever. Usernames can be removed only if they are optional in your instance settings and there's at least one other identifier which can be used for authentication.  This endpoint allows changing a user's password. When passing the `password` parameter directly you have two further options. You can ignore the password policy checks for your instance by setting the `skip_password_checks` parameter to `true`. You can also choose to sign the user out of all their active sessions on any device once the password is updated. Just set `sign_out_of_other_sessions` to `true`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user to update | [required] |
**update_user_request** | [**UpdateUserRequest**](UpdateUserRequest.md) |  | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_metadata

> crate::models::User update_user_metadata(user_id, update_user_metadata_request)
Merge and update a user's metadata

Update a user's metadata attributes by merging existing values with the provided parameters.  This endpoint behaves differently than the *Update a user* endpoint. Metadata values will not be replaced entirely. Instead, a deep merge will be performed. Deep means that any nested JSON objects will be merged as well.  You can remove metadata keys at any level by setting their value to `null`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user whose metadata will be updated and merged | [required] |
**update_user_metadata_request** | Option<[**UpdateUserMetadataRequest**](UpdateUserMetadataRequest.md)> |  |  |

### Return type

[**crate::models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_get_organization_memberships

> crate::models::OrganizationMemberships users_get_organization_memberships(user_id, limit, offset)
Retrieve all memberships for a user

Retrieve a paginated list of the user's organization memberships

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user whose organization memberships we want to retrieve | [required] |
**limit** | Option<**f32**> | Applies a limit to the number of results returned. Can be used for paginating the results together with `offset`. Must be an integer greater than zero and less than 500. By default, if not supplied, a limit of 10 is used. |  |[default to 10]
**offset** | Option<**f32**> | Skip the first `offset` results when paginating. Needs to be an integer greater or equal to zero. To be used in conjunction with `limit`. |  |[default to 0]

### Return type

[**crate::models::OrganizationMemberships**](OrganizationMemberships.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_password

> crate::models::VerifyPassword200Response verify_password(user_id, verify_password_request)
Verify the password of a user

Check that the user's password matches the supplied input. Useful for custom auth flows and re-verification.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user for whom to verify the password | [required] |
**verify_password_request** | Option<[**VerifyPasswordRequest**](VerifyPasswordRequest.md)> |  |  |

### Return type

[**crate::models::VerifyPassword200Response**](VerifyPassword_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_totp

> crate::models::VerifyTotp200Response verify_totp(user_id, verify_totp_request)
Verify a TOTP or backup code for a user

Verify that the provided TOTP or backup code is valid for the user. Verifying a backup code will result it in being consumed (i.e. it will become invalid). Useful for custom auth flows and re-verification.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user for whom to verify the TOTP | [required] |
**verify_totp_request** | Option<[**VerifyTotpRequest**](VerifyTotpRequest.md)> |  |  |

### Return type

[**crate::models::VerifyTotp200Response**](VerifyTOTP_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

