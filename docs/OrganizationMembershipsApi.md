# \OrganizationMembershipsApi

All URIs are relative to *https://api.clerk.dev/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_organization_membership**](OrganizationMembershipsApi.md#create_organization_membership) | **POST** /organizations/{organization_id}/memberships | Create a new organization membership
[**delete_organization_membership**](OrganizationMembershipsApi.md#delete_organization_membership) | **DELETE** /organizations/{organization_id}/memberships/{user_id} | Remove a member from an organization
[**list_organization_memberships**](OrganizationMembershipsApi.md#list_organization_memberships) | **GET** /organizations/{organization_id}/memberships | Get a list of all members of an organization
[**update_organization_membership**](OrganizationMembershipsApi.md#update_organization_membership) | **PATCH** /organizations/{organization_id}/memberships/{user_id} | Update an organization membership
[**update_organization_membership_metadata**](OrganizationMembershipsApi.md#update_organization_membership_metadata) | **PATCH** /organizations/{organization_id}/memberships/{user_id}/metadata | Merge and update organization membership metadata



## create_organization_membership

> crate::models::OrganizationMembership create_organization_membership(organization_id, create_organization_membership_request)
Create a new organization membership

Adds a user as a member to the given organization. Only users in the same instance as the organization can be added as members.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** | The ID of the organization where the new membership will be created | [required] |
**create_organization_membership_request** | [**CreateOrganizationMembershipRequest**](CreateOrganizationMembershipRequest.md) |  | [required] |

### Return type

[**crate::models::OrganizationMembership**](OrganizationMembership.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization_membership

> crate::models::OrganizationMembership delete_organization_membership(organization_id, user_id)
Remove a member from an organization

Removes the given membership from the organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** | The ID of the organization the membership belongs to | [required] |
**user_id** | **String** | The ID of the user that this membership belongs to | [required] |

### Return type

[**crate::models::OrganizationMembership**](OrganizationMembership.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_organization_memberships

> crate::models::OrganizationMemberships list_organization_memberships(organization_id, limit, offset)
Get a list of all members of an organization

Retrieves all user memberships for the given organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** | The organization ID. | [required] |
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


## update_organization_membership

> crate::models::OrganizationMembership update_organization_membership(organization_id, user_id, update_organization_membership_request)
Update an organization membership

Updates the properties of an existing organization membership

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** | The ID of the organization the membership belongs to | [required] |
**user_id** | **String** | The ID of the user that this membership belongs to | [required] |
**update_organization_membership_request** | [**UpdateOrganizationMembershipRequest**](UpdateOrganizationMembershipRequest.md) |  | [required] |

### Return type

[**crate::models::OrganizationMembership**](OrganizationMembership.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_organization_membership_metadata

> crate::models::OrganizationMembership update_organization_membership_metadata(organization_id, user_id, update_organization_membership_metadata_request)
Merge and update organization membership metadata

Update an organization membership's metadata attributes by merging existing values with the provided parameters. Metadata values will be updated via a deep merge. Deep means that any nested JSON objects will be merged as well. You can remove metadata keys at any level by setting their value to `null`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** | The ID of the organization the membership belongs to | [required] |
**user_id** | **String** | The ID of the user that this membership belongs to | [required] |
**update_organization_membership_metadata_request** | [**UpdateOrganizationMembershipMetadataRequest**](UpdateOrganizationMembershipMetadataRequest.md) |  | [required] |

### Return type

[**crate::models::OrganizationMembership**](OrganizationMembership.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

