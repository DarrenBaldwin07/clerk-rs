# \OrganizationPermissionsApi

All URIs are relative to *https://api.clerk.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_organization_permission**](OrganizationPermissionsApi.md#create_organization_permission) | **POST** /organization_permissions | Create a new organization permission
[**delete_organization_permission**](OrganizationPermissionsApi.md#delete_organization_permission) | **DELETE** /organization_permissions/{permission_id} | Delete an organization permission
[**get_organization_permission**](OrganizationPermissionsApi.md#get_organization_permission) | **GET** /organization_permissions/{permission_id} | Get an organization permission
[**list_organization_permissions**](OrganizationPermissionsApi.md#list_organization_permissions) | **GET** /organization_permissions | Get a list of all organization permissions
[**update_organization_permission**](OrganizationPermissionsApi.md#update_organization_permission) | **PATCH** /organization_permissions/{permission_id} | Update an organization permission



## create_organization_permission

> models::Permission create_organization_permission(create_organization_permission_request)
Create a new organization permission

Creates a new organization permission for the given instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_organization_permission_request** | [**CreateOrganizationPermissionRequest**](CreateOrganizationPermissionRequest.md) |  | [required] |

### Return type

[**models::Permission**](Permission.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization_permission

> models::DeletedObject delete_organization_permission(permission_id)
Delete an organization permission

Deletes an organization permission. System permissions cannot be deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**permission_id** | **String** | The ID of the permission to delete | [required] |

### Return type

[**models::DeletedObject**](DeletedObject.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_permission

> models::Permission get_organization_permission(permission_id)
Get an organization permission

Retrieves the details of an organization permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**permission_id** | **String** | The ID of the permission to retrieve | [required] |

### Return type

[**models::Permission**](Permission.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_organization_permissions

> models::Permissions list_organization_permissions(query, order_by, limit, offset)
Get a list of all organization permissions

Retrieves all organization permissions for the given instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | Option<**String**> | Returns organization permissions with ID, name, or key that match the given query. Uses exact match for permission ID and partial match for name and key. |  |
**order_by** | Option<**String**> | Allows to return organization permissions in a particular order. At the moment, you can order the returned permissions by their `created_at`, `name`, or `key`. In order to specify the direction, you can use the `+/-` symbols prepended in the property to order by. For example, if you want permissions to be returned in descending order according to their `created_at` property, you can use `-created_at`. |  |
**limit** | Option<**u32**> | Applies a limit to the number of results returned. Can be used for paginating the results together with `offset`. |  |[default to 10]
**offset** | Option<**u32**> | Skip the first `offset` results when paginating. Needs to be an integer greater or equal to zero. To be used in conjunction with `limit`. |  |[default to 0]

### Return type

[**models::Permissions**](Permissions.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_organization_permission

> models::Permission update_organization_permission(permission_id, update_organization_permission_request)
Update an organization permission

Updates the properties of an existing organization permission. System permissions cannot be updated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**permission_id** | **String** | The ID of the permission to update | [required] |
**update_organization_permission_request** | [**UpdateOrganizationPermissionRequest**](UpdateOrganizationPermissionRequest.md) |  | [required] |

### Return type

[**models::Permission**](Permission.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

