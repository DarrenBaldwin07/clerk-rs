# \UsersApi

All URIs are relative to *https://api.clerk.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**adjust_user_billing_credit_balance**](UsersApi.md#adjust_user_billing_credit_balance) | **POST** /users/{user_id}/billing/credits | Adjust a user's credit balance
[**ban_user**](UsersApi.md#ban_user) | **POST** /users/{user_id}/ban | Ban a user
[**create_user**](UsersApi.md#create_user) | **POST** /users | Create a new user
[**delete_backup_code**](UsersApi.md#delete_backup_code) | **DELETE** /users/{user_id}/backup_code | Disable all user's Backup codes
[**delete_external_account**](UsersApi.md#delete_external_account) | **DELETE** /users/{user_id}/external_accounts/{external_account_id} | Delete External Account
[**delete_totp**](UsersApi.md#delete_totp) | **DELETE** /users/{user_id}/totp | Delete all the user's TOTPs
[**delete_user**](UsersApi.md#delete_user) | **DELETE** /users/{user_id} | Delete a user
[**delete_user_profile_image**](UsersApi.md#delete_user_profile_image) | **DELETE** /users/{user_id}/profile_image | Delete user profile image
[**disable_mfa**](UsersApi.md#disable_mfa) | **DELETE** /users/{user_id}/mfa | Disable a user's MFA methods
[**get_o_auth_access_token**](UsersApi.md#get_o_auth_access_token) | **GET** /users/{user_id}/oauth_access_tokens/{provider} | Retrieve the OAuth access token of a user
[**get_user**](UsersApi.md#get_user) | **GET** /users/{user_id} | Retrieve a user
[**get_user_billing_credit_balance**](UsersApi.md#get_user_billing_credit_balance) | **GET** /users/{user_id}/billing/credits | Retrieve a user's credit balance
[**get_user_billing_subscription**](UsersApi.md#get_user_billing_subscription) | **GET** /users/{user_id}/billing/subscription | Retrieve a user's billing subscription
[**get_user_list**](UsersApi.md#get_user_list) | **GET** /users | List all users
[**get_users_count**](UsersApi.md#get_users_count) | **GET** /users/count | Count users
[**list_user_trusted_devices**](UsersApi.md#list_user_trusted_devices) | **GET** /users/{user_id}/trusted_devices | List a user's trusted devices
[**lock_user**](UsersApi.md#lock_user) | **POST** /users/{user_id}/lock | Lock a user
[**remove_user_password**](UsersApi.md#remove_user_password) | **POST** /users/{user_id}/remove_password | Remove a user's password
[**replace_user_metadata**](UsersApi.md#replace_user_metadata) | **PUT** /users/{user_id}/metadata | Replace a user's metadata
[**revoke_user_trusted_device**](UsersApi.md#revoke_user_trusted_device) | **DELETE** /users/{user_id}/trusted_devices/{trusted_device_id} | Revoke a user's trusted device
[**set_user_password_compromised**](UsersApi.md#set_user_password_compromised) | **POST** /users/{user_id}/password/set_compromised | Set a user's password as compromised
[**set_user_profile_image**](UsersApi.md#set_user_profile_image) | **POST** /users/{user_id}/profile_image | Set user profile image
[**unban_user**](UsersApi.md#unban_user) | **POST** /users/{user_id}/unban | Unban a user
[**unlock_user**](UsersApi.md#unlock_user) | **POST** /users/{user_id}/unlock | Unlock a user
[**unset_user_password_compromised**](UsersApi.md#unset_user_password_compromised) | **POST** /users/{user_id}/password/unset_compromised | Unset a user's password as compromised
[**update_user**](UsersApi.md#update_user) | **PATCH** /users/{user_id} | Update a user
[**update_user_metadata**](UsersApi.md#update_user_metadata) | **PATCH** /users/{user_id}/metadata | Merge and update a user's metadata
[**user_passkey_delete**](UsersApi.md#user_passkey_delete) | **DELETE** /users/{user_id}/passkeys/{passkey_identification_id} | Delete a user passkey
[**user_web3_wallet_delete**](UsersApi.md#user_web3_wallet_delete) | **DELETE** /users/{user_id}/web3_wallets/{web3_wallet_identification_id} | Delete a user web3 wallet
[**users_ban**](UsersApi.md#users_ban) | **POST** /users/ban | Ban multiple users
[**users_get_organization_invitations**](UsersApi.md#users_get_organization_invitations) | **GET** /users/{user_id}/organization_invitations | Retrieve all invitations for a user
[**users_get_organization_memberships**](UsersApi.md#users_get_organization_memberships) | **GET** /users/{user_id}/organization_memberships | Retrieve all memberships for a user
[**users_unban**](UsersApi.md#users_unban) | **POST** /users/unban | Unban multiple users
[**verify_password**](UsersApi.md#verify_password) | **POST** /users/{user_id}/verify_password | Verify the password of a user
[**verify_totp**](UsersApi.md#verify_totp) | **POST** /users/{user_id}/verify_totp | Verify a TOTP or backup code for a user



## adjust_user_billing_credit_balance

> models::CommerceCreditLedgerResponse adjust_user_billing_credit_balance(user_id, adjust_credit_balance_request)
Adjust a user's credit balance

Increases or decreases the credit balance for the specified user. Each adjustment is recorded as a ledger entry. The idempotency_key parameter ensures that duplicate requests are safely handled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user whose credit balance to adjust | [required] |
**adjust_credit_balance_request** | [**AdjustCreditBalanceRequest**](AdjustCreditBalanceRequest.md) | Parameters for the credit balance adjustment | [required] |

### Return type

[**models::CommerceCreditLedgerResponse**](CommerceCreditLedgerResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ban_user

> models::User ban_user(user_id)
Ban a user

Marks the given user as banned, which means that all their sessions are revoked and they are not allowed to sign in again.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user to ban | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user

> models::User create_user(create_user_request)
Create a new user

Creates a new user. Your user management settings determine how you should setup your user model.  By default, any email address and phone number created using this method is marked as verified. Use the `email_address_identification_status` and `phone_number_identification_status` arrays to instead create some or all of them as reserved (unverified but usable for sign-in and locked so no other user can claim them).  Note: If you are performing a migration, check out our guide on [zero downtime migrations](https://clerk.com/docs/deployments/migrate-overview).  The following rate limit rules apply to this endpoint: 1000 requests per 10 seconds for production instances and 100 requests per 10 seconds for development instances

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_user_request** | [**CreateUserRequest**](CreateUserRequest.md) |  | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_backup_code

> models::DisableMfa200Response delete_backup_code(user_id)
Disable all user's Backup codes

Disable all of a user's backup codes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user whose backup codes are to be deleted. | [required] |

### Return type

[**models::DisableMfa200Response**](DisableMFA_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_external_account

> models::DeletedObject delete_external_account(user_id, external_account_id)
Delete External Account

Delete an external account by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user's external account | [required] |
**external_account_id** | **String** | The ID of the external account to delete | [required] |

### Return type

[**models::DeletedObject**](DeletedObject.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_totp

> models::DisableMfa200Response delete_totp(user_id)
Delete all the user's TOTPs

Deletes all of the user's TOTPs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user whose TOTPs are to be deleted | [required] |

### Return type

[**models::DisableMfa200Response**](DisableMFA_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user

> models::DeletedObject delete_user(user_id)
Delete a user

Delete the specified user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user to delete | [required] |

### Return type

[**models::DeletedObject**](DeletedObject.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_profile_image

> models::User delete_user_profile_image(user_id)
Delete user profile image

Delete a user's profile image

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user to delete the profile image for | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_mfa

> models::DisableMfa200Response disable_mfa(user_id)
Disable a user's MFA methods

Disable all of a user's MFA methods (e.g. OTP sent via SMS, TOTP on their authenticator app) at once.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user whose MFA methods are to be disabled | [required] |

### Return type

[**models::DisableMfa200Response**](DisableMFA_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_o_auth_access_token

> Vec<models::OAuthAccessTokenInner> get_o_auth_access_token(user_id, provider, paginated, limit, offset)
Retrieve the OAuth access token of a user

Fetch the corresponding OAuth access token for a user that has previously authenticated with a particular OAuth provider. For OAuth 2.0, if the access token has expired and we have a corresponding refresh token, the access token will be refreshed transparently the new one will be returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user for which to retrieve the OAuth access token | [required] |
**provider** | **String** | The ID of the OAuth provider (e.g. `oauth_google`) | [required] |
**paginated** | Option<**bool**> | Whether to paginate the results. If true, the results will be paginated. If false, the results will not be paginated. |  |
**limit** | Option<**u32**> | Applies a limit to the number of results returned. Can be used for paginating the results together with `offset`. |  |[default to 10]
**offset** | Option<**u32**> | Skip the first `offset` results when paginating. Needs to be an integer greater or equal to zero. To be used in conjunction with `limit`. |  |[default to 0]

### Return type

[**Vec<models::OAuthAccessTokenInner>**](OAuthAccessToken_inner.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user

> models::User get_user(user_id)
Retrieve a user

Retrieve the details of a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user to retrieve | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_billing_credit_balance

> models::CommerceCreditBalanceResponse get_user_billing_credit_balance(user_id)
Retrieve a user's credit balance

Retrieves the current credit balance for the specified user. Credits can be applied during checkout to reduce the charge or automatically applied to upcoming recurring charges

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user whose credit balance to retrieve | [required] |

### Return type

[**models::CommerceCreditBalanceResponse**](CommerceCreditBalanceResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_billing_subscription

> models::CommerceSubscription get_user_billing_subscription(user_id)
Retrieve a user's billing subscription

Retrieves the billing subscription for the specified user. This includes subscription details, active plans, billing information, and payment status. The subscription contains subscription items which represent the individual plans the user is subscribed to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user whose subscription to retrieve | [required] |

### Return type

[**models::CommerceSubscription**](CommerceSubscription.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_list

> Vec<models::User> get_user_list(email_address, phone_number, external_id, username, web3_wallet, user_id, organization_id, query, email_address_query, phone_number_query, username_query, name_query, banned, last_active_at_before, last_active_at_after, last_active_at_since, created_at_before, created_at_after, last_sign_in_at_before, last_sign_in_at_after, provider, provider_user_id, limit, offset, starting_after, order_by)
List all users

Returns a list of all users. The users are returned sorted by creation date, with the newest users appearing first.  To walk more than a few pages, paginate with `starting_after` rather than `offset`. A cursor page costs the same no matter how far into the list it sits, while a large `offset` has to walk and discard every row before it, so it gets progressively slower and eventually times out. Cursor pagination requires the `created_at` ordering, which is the default.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_address** | Option<[**Vec<String>**](String.md)> | Returns users with the specified email addresses. Accepts up to 100 email addresses. Any email addresses not found are ignored. |  |
**phone_number** | Option<[**Vec<String>**](String.md)> | Returns users with the specified phone numbers. Accepts up to 100 phone numbers. Any phone numbers not found are ignored. |  |
**external_id** | Option<[**Vec<String>**](String.md)> | Returns users with the specified external IDs. For each external ID, the `+` and `-` can be prepended to the ID, which denote whether the respective external ID should be included or excluded from the result set. Accepts up to 100 external IDs. Any external IDs not found are ignored. |  |
**username** | Option<[**Vec<String>**](String.md)> | Returns users with the specified usernames. Accepts up to 100 usernames. Any usernames not found are ignored. |  |
**web3_wallet** | Option<[**Vec<String>**](String.md)> | Returns users with the specified web3 wallet addresses. Accepts up to 100 web3 wallet addresses. Any web3 wallet addresses not found are ignored. |  |
**user_id** | Option<[**Vec<String>**](String.md)> | Returns users with the user IDs specified. For each user ID, the `+` and `-` can be prepended to the ID, which denote whether the respective user ID should be included or excluded from the result set. Accepts up to 100 user IDs. Any user IDs not found are ignored. |  |
**organization_id** | Option<[**Vec<String>**](String.md)> | Returns users that have memberships to the given organizations. For each organization ID, the `+` and `-` can be prepended to the ID, which denote whether the respective organization should be included or excluded from the result set. Accepts up to 100 organization IDs. |  |
**query** | Option<**String**> | Returns users that match the given query. For possible matches, we check the email addresses, phone numbers, usernames, web3 wallets, user IDs, first and last names. The query value doesn't need to match the exact value you are looking for, it is capable of partial matches as well. |  |
**email_address_query** | Option<**String**> | Returns users with emails that match the given query, via case-insensitive partial match. For example, `email_address_query=ello` will match a user with the email `HELLO@example.com`. |  |
**phone_number_query** | Option<**String**> | Returns users with phone numbers that match the given query, via case-insensitive partial match. For example, `phone_number_query=555` will match a user with the phone number `+1555xxxxxxx`. |  |
**username_query** | Option<**String**> | Returns users with usernames that match the given query, via case-insensitive partial match. For example, `username_query=CoolUser` will match a user with the username `SomeCoolUser`. |  |
**name_query** | Option<**String**> | Returns users with names that match the given query, via case-insensitive partial match. |  |
**banned** | Option<**bool**> | Returns users which are either banned (`banned=true`) or not banned (`banned=false`). |  |
**last_active_at_before** | Option<**i32**> | Returns users whose last session activity was before the given date (with millisecond precision). Example: use 1700690400000 to retrieve users whose last session activity was before 2023-11-23. |  |
**last_active_at_after** | Option<**i32**> | Returns users whose last session activity was after the given date (with millisecond precision). Example: use 1700690400000 to retrieve users whose last session activity was after 2023-11-23. |  |
**last_active_at_since** | Option<**i32**> | Returns users that had session activity since the given date. Example: use 1700690400000 to retrieve users that had session activity from 2023-11-23 until the current day. Deprecated in favor of `last_active_at_after`. |  |
**created_at_before** | Option<**i32**> | Returns users who have been created before the given date (with millisecond precision). Example: use 1730160000000 to retrieve users who have been created before 2024-10-29. |  |
**created_at_after** | Option<**i32**> | Returns users who have been created after the given date (with millisecond precision). Example: use 1730160000000 to retrieve users who have been created after 2024-10-29. |  |
**last_sign_in_at_before** | Option<**i32**> | Returns users whose last sign-in was before the given date (with millisecond precision). Example: use 1700690400000 to retrieve users whose last sign-in was before 2023-11-23. |  |
**last_sign_in_at_after** | Option<**i32**> | Returns users whose last sign-in was after the given date (with millisecond precision). Example: use 1700690400000 to retrieve users whose last sign-in was after 2023-11-23. |  |
**provider** | Option<**String**> | Returns users with external accounts for the specified OAuth provider. Must be used in combination with the `provider_user_id` parameter. For example, use `provider=oauth_google&provider_user_id=12345` to retrieve a user with Google provider user ID 12345. |  |
**provider_user_id** | Option<[**Vec<String>**](String.md)> | Returns users with the specified provider user IDs for a specific provider. Must be used in combination with the `provider` parameter. For example, use `provider=oauth_google&provider_user_id=12345` to retrieve a user with Google provider user ID 12345. Accepts up to 100 provider user IDs. Any provider user IDs not found are ignored. |  |
**limit** | Option<**u32**> | Applies a limit to the number of results returned. Can be used for paginating the results together with `offset`. |  |[default to 10]
**offset** | Option<**u32**> | Skip the first `offset` results when paginating. Needs to be an integer greater or equal to zero. To be used in conjunction with `limit`. |  |[default to 0]
**starting_after** | Option<**String**> | A cursor for pagination: the `id` of the last user on the previous page. Returns the users that follow it.  **Requires ordering by `created_at`** — that is, `order_by` omitted, or set to `created_at`, `+created_at` or `-created_at`. Any other `order_by` value is rejected with a 422: the other orderings sort by a value that is neither unique per user nor immutable, so a cursor over them would skip or repeat users.  Cannot be combined with a non-zero `offset`, which is also a 422. Keep every other parameter identical across requests, and stop when a page returns fewer than `limit` users. |  |
**order_by** | Option<**String**> | Allows to return users in a particular order. At the moment, you can order the returned users by their `created_at`,`updated_at`,`email_address`,`web3wallet`,`first_name`,`last_name`,`phone_number`,`username`,`last_active_at`,`last_sign_in_at`. In order to specify the direction, you can use the `+/-` symbols prepended in the property to order by. For example, if you want users to be returned in descending order according to their `created_at` property, you can use `-created_at`. If you don't use `+` or `-`, then `+` is implied. We only support one `order_by` parameter, and if multiple `order_by` parameters are provided, we will only keep the first one. For example, if you pass `order_by=username&order_by=created_at`, we will consider only the first `order_by` parameter, which is `username`. The `created_at` parameter will be ignored in this case. Only the `created_at` orderings can be combined with `starting_after` cursor pagination; see that parameter. |  |[default to -created_at]

### Return type

[**Vec<models::User>**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_count

> models::TotalCount get_users_count(email_address, phone_number, external_id, username, web3_wallet, user_id, organization_id, query, email_address_query, phone_number_query, username_query, name_query, banned, last_active_at_before, last_active_at_after, last_active_at_since, created_at_before, created_at_after, last_sign_in_at_before, last_sign_in_at_after, provider, provider_user_id)
Count users

Returns a total count of all users that match the given filtering criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_address** | Option<[**Vec<String>**](String.md)> | Counts users with the specified email addresses. Accepts up to 100 email addresses. Any email addresses not found are ignored. |  |
**phone_number** | Option<[**Vec<String>**](String.md)> | Counts users with the specified phone numbers. Accepts up to 100 phone numbers. Any phone numbers not found are ignored. |  |
**external_id** | Option<[**Vec<String>**](String.md)> | Counts users with the specified external IDs. Accepts up to 100 external IDs. Any external IDs not found are ignored. |  |
**username** | Option<[**Vec<String>**](String.md)> | Counts users with the specified usernames. Accepts up to 100 usernames. Any usernames not found are ignored. |  |
**web3_wallet** | Option<[**Vec<String>**](String.md)> | Counts users with the specified web3 wallet addresses. Accepts up to 100 web3 wallet addresses. Any web3 wallet addresses not found are ignored. |  |
**user_id** | Option<[**Vec<String>**](String.md)> | Counts users with the user IDs specified. Accepts up to 100 user IDs. Any user IDs not found are ignored. |  |
**organization_id** | Option<[**Vec<String>**](String.md)> | Returns users that have memberships to the given organizations. For each organization ID, the `+` and `-` can be prepended to the ID, which denote whether the respective organization should be included or excluded from the result set. Accepts up to 100 organization IDs. |  |
**query** | Option<**String**> | Counts users that match the given query. For possible matches, we check the email addresses, phone numbers, usernames, web3 wallets, user IDs, first and last names. The query value doesn't need to match the exact value you are looking for, it is capable of partial matches as well. |  |
**email_address_query** | Option<**String**> | Counts users with emails that match the given query, via case-insensitive partial match. For example, `email_address_query=ello` will match a user with the email `HELLO@example.com`, and will be included in the resulting count. |  |
**phone_number_query** | Option<**String**> | Counts users with phone numbers that match the given query, via case-insensitive partial match. For example, `phone_number_query=555` will match a user with the phone number `+1555xxxxxxx`, and will be included in the resulting count. |  |
**username_query** | Option<**String**> | Counts users with usernames that match the given query, via case-insensitive partial match. For example, `username_query=CoolUser` will match a user with the username `SomeCoolUser`, and will be included in the resulting count. |  |
**name_query** | Option<**String**> | Returns users with names that match the given query, via case-insensitive partial match. |  |
**banned** | Option<**bool**> | Counts users which are either banned (`banned=true`) or not banned (`banned=false`). |  |
**last_active_at_before** | Option<**i32**> | Returns users whose last session activity was before the given date (with millisecond precision). Example: use 1700690400000 to retrieve users whose last session activity was before 2023-11-23. |  |
**last_active_at_after** | Option<**i32**> | Returns users whose last session activity was after the given date (with millisecond precision). Example: use 1700690400000 to retrieve users whose last session activity was after 2023-11-23. |  |
**last_active_at_since** | Option<**i32**> | Returns users that had session activity since the given date. Example: use 1700690400000 to retrieve users that had session activity from 2023-11-23 until the current day. Deprecated in favor of `last_active_at_after`. |  |
**created_at_before** | Option<**i32**> | Returns users who have been created before the given date (with millisecond precision). Example: use 1730160000000 to retrieve users who have been created before 2024-10-29. |  |
**created_at_after** | Option<**i32**> | Returns users who have been created after the given date (with millisecond precision). Example: use 1730160000000 to retrieve users who have been created after 2024-10-29. |  |
**last_sign_in_at_before** | Option<**i32**> | Counts users whose last sign-in was before the given date (with millisecond precision). Example: use 1700690400000 to count users whose last sign-in was before 2023-11-23. |  |
**last_sign_in_at_after** | Option<**i32**> | Counts users whose last sign-in was after the given date (with millisecond precision). Example: use 1700690400000 to count users whose last sign-in was after 2023-11-23. |  |
**provider** | Option<**String**> | Counts users with external accounts for the specified OAuth provider. Must be used in combination with the `provider_user_id` parameter. For example, use `provider=oauth_google&provider_user_id=12345` to count users with Google provider user ID 12345. Accepts up to 100 providers. |  |
**provider_user_id** | Option<[**Vec<String>**](String.md)> | Counts users with the specified provider user IDs for a specific provider. Must be used in combination with the `provider` parameter. For example, use `provider=oauth_google&provider_user_id=12345` to count users with Google provider user ID 12345. Accepts up to 100 provider user IDs. Any provider user IDs not found are ignored. |  |

### Return type

[**models::TotalCount**](TotalCount.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_user_trusted_devices

> models::ListUserTrustedDevices200Response list_user_trusted_devices(user_id)
List a user's trusted devices

Returns the active trusted devices enrolled by the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user whose trusted devices are returned | [required] |

### Return type

[**models::ListUserTrustedDevices200Response**](ListUserTrustedDevices_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lock_user

> models::User lock_user(user_id)
Lock a user

Marks the given user as locked, which means they are not allowed to sign in again until the lock expires. Lock duration can be configured in the instance's restrictions settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user to lock | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_user_password

> models::User remove_user_password(user_id, remove_user_password_request)
Remove a user's password

Removes the password credential from the given user. This is a privileged operation and does not require the user's current password. Password removal is allowed even when the user has no other sign-in method configured.  If the user does not have a password, the user is returned unchanged and no password-deletion or user-update event is emitted. By default, existing sessions remain active. Set `sign_out_of_other_sessions` to `true` to revoke sessions active when the request is processed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user whose password to remove | [required] |
**remove_user_password_request** | Option<[**RemoveUserPasswordRequest**](RemoveUserPasswordRequest.md)> |  |  |

### Return type

[**models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_user_metadata

> models::User replace_user_metadata(user_id, replace_user_metadata_request)
Replace a user's metadata

Replace a user's metadata attributes with the provided values.  Unlike `PATCH /v1/users/{user_id}/metadata` (merge semantics), this endpoint replaces the supplied metadata fields entirely — the prior contents of each supplied field are discarded. Fields omitted from the request body are left unchanged.  Prefer the `PATCH` endpoint for partial updates. Use `PUT` only when you explicitly intend to overwrite a metadata field wholesale.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user whose metadata will be replaced | [required] |
**replace_user_metadata_request** | Option<[**ReplaceUserMetadataRequest**](ReplaceUserMetadataRequest.md)> |  |  |

### Return type

[**models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_user_trusted_device

> models::TrustedDevice revoke_user_trusted_device(user_id, trusted_device_id)
Revoke a user's trusted device

Revokes an active trusted device enrolled by the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user that owns the trusted device | [required] |
**trusted_device_id** | **String** | The ID of the trusted device to revoke | [required] |

### Return type

[**models::TrustedDevice**](TrustedDevice.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_user_password_compromised

> models::User set_user_password_compromised(user_id, set_user_password_compromised_request)
Set a user's password as compromised

Sets the given user's password as compromised. The user will be prompted to reset their password on their next sign-in.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user to set the password as compromised | [required] |
**set_user_password_compromised_request** | Option<[**SetUserPasswordCompromisedRequest**](SetUserPasswordCompromisedRequest.md)> |  |  |

### Return type

[**models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_user_profile_image

> models::User set_user_profile_image(user_id, file)
Set user profile image

Update a user's profile image

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user to update the profile image for | [required] |
**file** | Option<**std::path::PathBuf**> |  |  |

### Return type

[**models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unban_user

> models::User unban_user(user_id)
Unban a user

Removes the ban mark from the given user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user to unban | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlock_user

> models::User unlock_user(user_id)
Unlock a user

Removes the lock from the given user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user to unlock | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unset_user_password_compromised

> models::User unset_user_password_compromised(user_id)
Unset a user's password as compromised

Sets the given user's password as no longer compromised. The user will no longer be prompted to reset their password on their next sign-in.  If the user is in reserved-email password quarantine, the quarantine is preserved and the returned user will still have `requires_password_reset` set to `true`. Reserved-email password quarantine can only be cleared by completing a password reset or changing/removing the password.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user to unset the compromised status for | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> models::User update_user(user_id, update_user_request)
Update a user

Update a user's attributes.  You can set the user's primary contact identifiers (email address and phone numbers) by updating the `primary_email_address_id` and `primary_phone_number_id` attributes respectively. Both IDs should correspond to verified identifications that belong to the user.  You can remove a user's username by setting the username attribute to null or the blank string \"\".  As of API version 2026-05-12, this endpoint no longer accepts `public_metadata`, `private_metadata`, or `unsafe_metadata`. Use `PATCH /v1/users/{user_id}/metadata` to merge updates into existing metadata, or `PUT /v1/users/{user_id}/metadata` to replace a metadata field entirely.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user to update | [required] |
**update_user_request** | [**UpdateUserRequest**](UpdateUserRequest.md) |  | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_metadata

> models::User update_user_metadata(user_id, update_user_metadata_request)
Merge and update a user's metadata

Update a user's metadata attributes by merging existing values with the provided parameters.  This endpoint behaves differently than the *Update a user* endpoint. Metadata values will not be replaced entirely. Instead, a deep merge will be performed. Deep means that any nested JSON objects will be merged as well.  You can remove metadata keys at any level by setting their value to `null`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user whose metadata will be updated and merged | [required] |
**update_user_metadata_request** | Option<[**UpdateUserMetadataRequest**](UpdateUserMetadataRequest.md)> |  |  |

### Return type

[**models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_passkey_delete

> models::DeletedObject user_passkey_delete(user_id, passkey_identification_id)
Delete a user passkey

Delete the passkey identification for a given user and notify them through email.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user that owns the passkey identity | [required] |
**passkey_identification_id** | **String** | The ID of the passkey identity to be deleted | [required] |

### Return type

[**models::DeletedObject**](DeletedObject.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_web3_wallet_delete

> models::DeletedObject user_web3_wallet_delete(user_id, web3_wallet_identification_id)
Delete a user web3 wallet

Delete the web3 wallet identification for a given user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user that owns the web3 wallet | [required] |
**web3_wallet_identification_id** | **String** | The ID of the web3 wallet identity to be deleted | [required] |

### Return type

[**models::DeletedObject**](DeletedObject.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_ban

> Vec<models::User> users_ban(users_ban_request)
Ban multiple users

Marks multiple users as banned, which means that all their sessions are revoked and they are not allowed to sign in again.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**users_ban_request** | [**UsersBanRequest**](UsersBanRequest.md) |  | [required] |

### Return type

[**Vec<models::User>**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_get_organization_invitations

> models::OrganizationInvitationsWithPublicOrganizationData users_get_organization_invitations(user_id, limit, offset, status)
Retrieve all invitations for a user

Retrieve a paginated list of the user's organization invitations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user whose organization invitations we want to retrieve | [required] |
**limit** | Option<**u32**> | Applies a limit to the number of results returned. Can be used for paginating the results together with `offset`. |  |[default to 10]
**offset** | Option<**u32**> | Skip the first `offset` results when paginating. Needs to be an integer greater or equal to zero. To be used in conjunction with `limit`. |  |[default to 0]
**status** | Option<**String**> | Filter organization invitations based on their status |  |

### Return type

[**models::OrganizationInvitationsWithPublicOrganizationData**](OrganizationInvitationsWithPublicOrganizationData.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_get_organization_memberships

> models::OrganizationMemberships users_get_organization_memberships(user_id, limit, offset)
Retrieve all memberships for a user

Retrieve a paginated list of the user's organization memberships

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user whose organization memberships we want to retrieve | [required] |
**limit** | Option<**u32**> | Applies a limit to the number of results returned. Can be used for paginating the results together with `offset`. |  |[default to 10]
**offset** | Option<**u32**> | Skip the first `offset` results when paginating. Needs to be an integer greater or equal to zero. To be used in conjunction with `limit`. |  |[default to 0]

### Return type

[**models::OrganizationMemberships**](OrganizationMemberships.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_unban

> Vec<models::User> users_unban(users_unban_request)
Unban multiple users

Removes the ban mark from multiple users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**users_unban_request** | [**UsersUnbanRequest**](UsersUnbanRequest.md) |  | [required] |

### Return type

[**Vec<models::User>**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_password

> models::VerifyPassword200Response verify_password(user_id, verify_password_request)
Verify the password of a user

Check that the user's password matches the supplied input. Useful for custom auth flows and re-verification.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user for whom to verify the password | [required] |
**verify_password_request** | Option<[**VerifyPasswordRequest**](VerifyPasswordRequest.md)> |  |  |

### Return type

[**models::VerifyPassword200Response**](VerifyPassword_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_totp

> models::VerifyTotp200Response verify_totp(user_id, verify_totp_request)
Verify a TOTP or backup code for a user

Verify that the provided TOTP or backup code is valid for the user. Verifying a backup code will result it in being consumed (i.e. it will become invalid). Useful for custom auth flows and re-verification.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user for whom to verify the TOTP | [required] |
**verify_totp_request** | Option<[**VerifyTotpRequest**](VerifyTotpRequest.md)> |  |  |

### Return type

[**models::VerifyTotp200Response**](VerifyTOTP_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

