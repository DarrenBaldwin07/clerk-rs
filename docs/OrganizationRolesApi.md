# \OrganizationRolesApi

All URIs are relative to *https://api.clerk.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**assign_permission_to_organization_role**](OrganizationRolesApi.md#assign_permission_to_organization_role) | **POST** /organization_roles/{organization_role_id}/permissions/{permission_id} | Assign a permission to an organization role
[**create_organization_role**](OrganizationRolesApi.md#create_organization_role) | **POST** /organization_roles | Create an organization role
[**delete_organization_role**](OrganizationRolesApi.md#delete_organization_role) | **DELETE** /organization_roles/{organization_role_id} | Delete an organization role
[**get_organization_role**](OrganizationRolesApi.md#get_organization_role) | **GET** /organization_roles/{organization_role_id} | Retrieve an organization role
[**list_organization_roles**](OrganizationRolesApi.md#list_organization_roles) | **GET** /organization_roles | Get a list of organization roles
[**remove_permission_from_organization_role**](OrganizationRolesApi.md#remove_permission_from_organization_role) | **DELETE** /organization_roles/{organization_role_id}/permissions/{permission_id} | Remove a permission from an organization role
[**update_organization_role**](OrganizationRolesApi.md#update_organization_role) | **PATCH** /organization_roles/{organization_role_id} | Update an organization role



## assign_permission_to_organization_role

> models::Role assign_permission_to_organization_role(organization_role_id, permission_id)
Assign a permission to an organization role

Assigns a permission to an organization role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_role_id** | **String** | The ID of the organization role | [required] |
**permission_id** | **String** | The ID of the permission to assign | [required] |

### Return type

[**models::Role**](Role.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_organization_role

> models::Role create_organization_role(create_organization_role_request)
Create an organization role

Creates a new organization role with the given name and permissions for an instance. The key must be unique for the instance and start with the 'org:' prefix, followed by lowercase alphanumeric characters and underscores only. You can optionally provide a description for the role and specify whether it should be included in the initial role set. Organization roles support permissions that can be assigned to control access within the organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_organization_role_request** | [**CreateOrganizationRoleRequest**](CreateOrganizationRoleRequest.md) |  | [required] |

### Return type

[**models::Role**](Role.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization_role

> models::DeletedObject delete_organization_role(organization_role_id)
Delete an organization role

Deletes the organization role. The role cannot be deleted if it is currently used as the default creator role, domain default role, assigned to any members, or exists in any invitations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_role_id** | **String** | The ID of the organization role to delete | [required] |

### Return type

[**models::DeletedObject**](DeletedObject.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_role

> models::Role get_organization_role(organization_role_id)
Retrieve an organization role

Use this request to retrieve an existing organization role by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_role_id** | **String** | The ID of the organization role | [required] |

### Return type

[**models::Role**](Role.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_organization_roles

> models::Roles list_organization_roles(query, order_by, limit, offset)
Get a list of organization roles

This request returns the list of organization roles for the instance. Results can be paginated using the optional `limit` and `offset` query parameters. The organization roles are ordered by descending creation date. Most recent roles will be returned first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | Option<**String**> | Returns organization roles with ID, name, or key that match the given query. Uses exact match for organization role ID and partial match for name and key. |  |
**order_by** | Option<**String**> | Allows to return organization roles in a particular order. At the moment, you can order the returned organization roles by their `created_at`, `name`, or `key`. In order to specify the direction, you can use the `+/-` symbols prepended in the property to order by. For example, if you want organization roles to be returned in descending order according to their `created_at` property, you can use `-created_at`. If you don't use `+` or `-`, then `+` is implied. Defaults to `-created_at`. |  |[default to -created_at]
**limit** | Option<**u32**> | Applies a limit to the number of results returned. Can be used for paginating the results together with `offset`. |  |[default to 10]
**offset** | Option<**u32**> | Skip the first `offset` results when paginating. Needs to be an integer greater or equal to zero. To be used in conjunction with `limit`. |  |[default to 0]

### Return type

[**models::Roles**](Roles.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_permission_from_organization_role

> models::Role remove_permission_from_organization_role(organization_role_id, permission_id)
Remove a permission from an organization role

Removes a permission from an organization role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_role_id** | **String** | The ID of the organization role | [required] |
**permission_id** | **String** | The ID of the permission to remove | [required] |

### Return type

[**models::Role**](Role.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_organization_role

> models::Role update_organization_role(organization_role_id, update_organization_role_request)
Update an organization role

Updates an existing organization role. You can update the name, key, description, and permissions of the role. All parameters are optional - you can update only the fields you want to change. If the role is used as a creator role or domain default role, updating the key will cascade the update to the organization settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_role_id** | **String** | The ID of the organization role to update | [required] |
**update_organization_role_request** | [**UpdateOrganizationRoleRequest**](UpdateOrganizationRoleRequest.md) |  | [required] |

### Return type

[**models::Role**](Role.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

