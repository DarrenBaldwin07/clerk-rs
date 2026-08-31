# \OrganizationMembershipsApi

All URIs are relative to *https://api.clerk.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_organization_membership**](OrganizationMembershipsApi.md#create_organization_membership) | **POST** /organizations/{organization_id}/memberships | Create a new organization membership
[**delete_organization_membership**](OrganizationMembershipsApi.md#delete_organization_membership) | **DELETE** /organizations/{organization_id}/memberships/{user_id} | Remove a member from an organization
[**instance_get_organization_memberships**](OrganizationMembershipsApi.md#instance_get_organization_memberships) | **GET** /organization_memberships | Get a list of all organization memberships within an instance.
[**list_organization_memberships**](OrganizationMembershipsApi.md#list_organization_memberships) | **GET** /organizations/{organization_id}/memberships | Get a list of all members of an organization
[**update_organization_membership**](OrganizationMembershipsApi.md#update_organization_membership) | **PATCH** /organizations/{organization_id}/memberships/{user_id} | Update an organization membership
[**update_organization_membership_metadata**](OrganizationMembershipsApi.md#update_organization_membership_metadata) | **PATCH** /organizations/{organization_id}/memberships/{user_id}/metadata | Merge and update organization membership metadata



## create_organization_membership

> models::OrganizationMembership create_organization_membership(organization_id, create_organization_membership_request)
Create a new organization membership

Adds a user as a member to the given organization. Only users in the same instance as the organization can be added as members.  This organization will be the user's [active organization] (https://clerk.com/docs/organizations/overview#active-organization) the next time they create a session, presuming they don't explicitly set a different organization as active before then.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** | The ID of the organization where the new membership will be created | [required] |
**create_organization_membership_request** | [**CreateOrganizationMembershipRequest**](CreateOrganizationMembershipRequest.md) |  | [required] |

### Return type

[**models::OrganizationMembership**](OrganizationMembership.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization_membership

> models::OrganizationMembership delete_organization_membership(organization_id, user_id)
Remove a member from an organization

Removes the given membership from the organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** | The ID of the organization to which this membership belongs | [required] |
**user_id** | **String** | The ID of the user to which this membership belongs | [required] |

### Return type

[**models::OrganizationMembership**](OrganizationMembership.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instance_get_organization_memberships

> models::OrganizationMemberships instance_get_organization_memberships(order_by, limit, offset)
Get a list of all organization memberships within an instance.

Retrieves all organization user memberships for the given instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_by** | Option<**String**> | Sorts organizations memberships by phone_number, email_address, created_at, first_name, last_name or username. By prepending one of those values with + or -, we can choose to sort in ascending (ASC) or descending (DESC) order. |  |
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


## list_organization_memberships

> models::OrganizationMemberships list_organization_memberships(organization_id, order_by, user_id, email_address, phone_number, username, web3_wallet, role, query, email_address_query, phone_number_query, username_query, name_query, last_active_at_before, last_active_at_after, created_at_before, created_at_after, limit, offset)
Get a list of all members of an organization

Retrieves all user memberships for the given organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** | The organization ID. | [required] |
**order_by** | Option<**String**> | Sorts organizations memberships by phone_number, email_address, created_at, first_name, last_name or username. By prepending one of those values with + or -, we can choose to sort in ascending (ASC) or descending (DESC) order.\" |  |
**user_id** | Option<[**Vec<String>**](String.md)> | Returns users with the user IDs specified. For each user ID, the `+` and `-` can be prepended to the ID, which denote whether the respective user ID should be included or excluded from the result set. Accepts up to 100 user IDs. Any user IDs not found are ignored. |  |
**email_address** | Option<[**Vec<String>**](String.md)> | Returns users with the specified email addresses. Accepts up to 100 email addresses. Any email addresses not found are ignored. |  |
**phone_number** | Option<[**Vec<String>**](String.md)> | Returns users with the specified phone numbers. Accepts up to 100 phone numbers. Any phone numbers not found are ignored. |  |
**username** | Option<[**Vec<String>**](String.md)> | Returns users with the specified usernames. Accepts up to 100 usernames. Any usernames not found are ignored. |  |
**web3_wallet** | Option<[**Vec<String>**](String.md)> | Returns users with the specified web3 wallet addresses. Accepts up to 100 web3 wallet addresses. Any web3 wallet addresses not found are ignored. |  |
**role** | Option<[**Vec<String>**](String.md)> | Returns users with the specified roles. Accepts up to 100 roles. Any roles not found are ignored. |  |
**query** | Option<**String**> | Returns users that match the given query. For possible matches, we check the email addresses, phone numbers, usernames, web3 wallets, user IDs, first and last names. The query value doesn't need to match the exact value you are looking for, it is capable of partial matches as well. |  |
**email_address_query** | Option<**String**> | Returns users with emails that match the given query, via case-insensitive partial match. For example, `email_address_query=ello` will match a user with the email `HELLO@example.com`. |  |
**phone_number_query** | Option<**String**> | Returns users with phone numbers that match the given query, via case-insensitive partial match. For example, `phone_number_query=555` will match a user with the phone number `+1555xxxxxxx`. |  |
**username_query** | Option<**String**> | Returns users with usernames that match the given query, via case-insensitive partial match. For example, `username_query=CoolUser` will match a user with the username `SomeCoolUser`. |  |
**name_query** | Option<**String**> | Returns users with names that match the given query, via case-insensitive partial match. |  |
**last_active_at_before** | Option<**i64**> | Returns users whose last session activity was before the given date (with millisecond precision). Example: use 1700690400000 to retrieve users whose last session activity was before 2023-11-23. |  |
**last_active_at_after** | Option<**i64**> | Returns users whose last session activity was after the given date (with millisecond precision). Example: use 1700690400000 to retrieve users whose last session activity was after 2023-11-23. |  |
**created_at_before** | Option<**i64**> | Returns users who have been created before the given date (with millisecond precision). Example: use 1730160000000 to retrieve users who have been created before 2024-10-29. |  |
**created_at_after** | Option<**i64**> | Returns users who have been created after the given date (with millisecond precision). Example: use 1730160000000 to retrieve users who have been created after 2024-10-29. |  |
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


## update_organization_membership

> models::OrganizationMembership update_organization_membership(organization_id, user_id, update_organization_membership_request)
Update an organization membership

Updates the properties of an existing organization membership

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** | The ID of the organization to which this membership belongs | [required] |
**user_id** | **String** | The ID of the user to which this membership belongs | [required] |
**update_organization_membership_request** | [**UpdateOrganizationMembershipRequest**](UpdateOrganizationMembershipRequest.md) |  | [required] |

### Return type

[**models::OrganizationMembership**](OrganizationMembership.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_organization_membership_metadata

> models::OrganizationMembership update_organization_membership_metadata(organization_id, user_id, update_organization_membership_metadata_request)
Merge and update organization membership metadata

Update an organization membership's metadata attributes by merging existing values with the provided parameters. Metadata values will be updated via a deep merge. Deep means that any nested JSON objects will be merged as well. You can remove metadata keys at any level by setting their value to `null`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** | The ID of the organization to which this membership belongs | [required] |
**user_id** | **String** | The ID of the user to which this membership belongs | [required] |
**update_organization_membership_metadata_request** | Option<[**UpdateOrganizationMembershipMetadataRequest**](UpdateOrganizationMembershipMetadataRequest.md)> |  |  |

### Return type

[**models::OrganizationMembership**](OrganizationMembership.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

