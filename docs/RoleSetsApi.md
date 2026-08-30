# \RoleSetsApi

All URIs are relative to *https://api.clerk.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_roles_to_role_set**](RoleSetsApi.md#add_roles_to_role_set) | **POST** /role_sets/{role_set_key_or_id}/roles | Add roles to a role set
[**create_role_set**](RoleSetsApi.md#create_role_set) | **POST** /role_sets | Create a role set
[**get_role_set**](RoleSetsApi.md#get_role_set) | **GET** /role_sets/{role_set_key_or_id} | Retrieve a role set
[**list_role_sets**](RoleSetsApi.md#list_role_sets) | **GET** /role_sets | Get a list of role sets
[**replace_role_in_role_set**](RoleSetsApi.md#replace_role_in_role_set) | **POST** /role_sets/{role_set_key_or_id}/roles/replace | Replace a role in a role set
[**replace_role_set**](RoleSetsApi.md#replace_role_set) | **POST** /role_sets/{role_set_key_or_id}/replace | Replace a role set
[**update_role_set**](RoleSetsApi.md#update_role_set) | **PATCH** /role_sets/{role_set_key_or_id} | Update a role set



## add_roles_to_role_set

> models::RoleSet add_roles_to_role_set(role_set_key_or_id, add_roles_to_role_set_request)
Add roles to a role set

Adds one or more roles to an existing role set. You can optionally update the default role or creator role when adding new roles.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_set_key_or_id** | **String** | The key or ID of the role set | [required] |
**add_roles_to_role_set_request** | [**AddRolesToRoleSetRequest**](AddRolesToRoleSetRequest.md) |  | [required] |

### Return type

[**models::RoleSet**](RoleSet.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_role_set

> models::RoleSet create_role_set(create_role_set_request)
Create a role set

Creates a new role set with the given name and roles. The key must be unique for the instance and start with the 'role_set:' prefix, followed by lowercase alphanumeric characters and underscores only. You must provide at least one role and specify a default role key and creator role key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_role_set_request** | [**CreateRoleSetRequest**](CreateRoleSetRequest.md) |  | [required] |

### Return type

[**models::RoleSet**](RoleSet.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_role_set

> models::RoleSet get_role_set(role_set_key_or_id)
Retrieve a role set

Retrieves an existing role set by its key or ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_set_key_or_id** | **String** | The key or ID of the role set | [required] |

### Return type

[**models::RoleSet**](RoleSet.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_role_sets

> models::RoleSets list_role_sets(query, order_by, limit, offset)
Get a list of role sets

Returns a list of role sets for the instance. Results can be paginated using the optional `limit` and `offset` query parameters. The role sets are ordered by descending creation date by default.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | Option<**String**> | Returns role sets with ID, name, or key that match the given query. Uses exact match for role set ID and partial match for name and key. |  |
**order_by** | Option<**String**> | Allows to return role sets in a particular order. At the moment, you can order the returned role sets by their `created_at`, `name`, or `key`. In order to specify the direction, you can use the `+/-` symbols prepended in the property to order by. For example, if you want role sets to be returned in descending order according to their `created_at` property, you can use `-created_at`. If you don't use `+` or `-`, then `+` is implied. Defaults to `-created_at`. |  |[default to -created_at]
**limit** | Option<**u32**> | Applies a limit to the number of results returned. Can be used for paginating the results together with `offset`. |  |[default to 10]
**offset** | Option<**u32**> | Skip the first `offset` results when paginating. Needs to be an integer greater or equal to zero. To be used in conjunction with `limit`. |  |[default to 0]

### Return type

[**models::RoleSets**](RoleSets.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_role_in_role_set

> models::RoleSet replace_role_in_role_set(role_set_key_or_id, replace_role_in_role_set_request)
Replace a role in a role set

Replaces a role in a role set with another role. This atomically removes the source role and reassigns any members to the destination role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_set_key_or_id** | **String** | The key or ID of the role set | [required] |
**replace_role_in_role_set_request** | [**ReplaceRoleInRoleSetRequest**](ReplaceRoleInRoleSetRequest.md) |  | [required] |

### Return type

[**models::RoleSet**](RoleSet.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_role_set

> models::DeletedObject replace_role_set(role_set_key_or_id, replace_role_set_request)
Replace a role set

Replaces a role set with another role set. This is functionally equivalent to deleting the role set but allows for atomic replacement with migration support. Organizations using this role set will be migrated to the destination role set.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_set_key_or_id** | **String** | The key or ID of the role set to replace | [required] |
**replace_role_set_request** | [**ReplaceRoleSetRequest**](ReplaceRoleSetRequest.md) |  | [required] |

### Return type

[**models::DeletedObject**](DeletedObject.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_role_set

> models::RoleSet update_role_set(role_set_key_or_id, update_role_set_request)
Update a role set

Updates an existing role set. You can update the name, key, description, type, default role, or creator role. All parameters are optional - you can update only the fields you want to change.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_set_key_or_id** | **String** | The key or ID of the role set to update | [required] |
**update_role_set_request** | [**UpdateRoleSetRequest**](UpdateRoleSetRequest.md) |  | [required] |

### Return type

[**models::RoleSet**](RoleSet.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

