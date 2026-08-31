# \ScimDirectoriesApi

All URIs are relative to *https://api.clerk.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_scim_directory**](ScimDirectoriesApi.md#create_scim_directory) | **POST** /scim_directories | Create a directory
[**create_scim_group_role_mapping**](ScimDirectoriesApi.md#create_scim_group_role_mapping) | **POST** /scim_directories/{scim_directory_id}/group_role_mappings | Create a SCIM group role mapping
[**delete_scim_directory**](ScimDirectoriesApi.md#delete_scim_directory) | **DELETE** /scim_directories/{scim_directory_id} | Delete a directory
[**delete_scim_group_role_mapping**](ScimDirectoriesApi.md#delete_scim_group_role_mapping) | **DELETE** /scim_directories/{scim_directory_id}/group_role_mappings/{mapping_id} | Delete a SCIM group role mapping
[**get_scim_directory**](ScimDirectoriesApi.md#get_scim_directory) | **GET** /scim_directories/{scim_directory_id} | Retrieve a directory
[**list_scim_directories**](ScimDirectoriesApi.md#list_scim_directories) | **GET** /scim_directories | List all directories
[**list_scim_group_role_mappings**](ScimDirectoriesApi.md#list_scim_group_role_mappings) | **GET** /scim_directories/{scim_directory_id}/group_role_mappings | List SCIM group role mappings
[**replace_scim_group_role_mappings**](ScimDirectoriesApi.md#replace_scim_group_role_mappings) | **PUT** /scim_directories/{scim_directory_id}/group_role_mappings | Replace SCIM group role mappings
[**rotate_scim_directory_api_key**](ScimDirectoriesApi.md#rotate_scim_directory_api_key) | **POST** /scim_directories/{scim_directory_id}/rotate_api_key | Rotate a directory's API key
[**update_scim_directory**](ScimDirectoriesApi.md#update_scim_directory) | **PATCH** /scim_directories/{scim_directory_id} | Update a directory



## create_scim_directory

> models::ScimDirectory create_scim_directory(create_scim_directory_request)
Create a directory

Create a new directory for the instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_scim_directory_request** | Option<[**CreateScimDirectoryRequest**](CreateScimDirectoryRequest.md)> |  |  |

### Return type

[**models::ScimDirectory**](SCIMDirectory.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_scim_group_role_mapping

> models::ScimGroupRoleMapping create_scim_group_role_mapping(scim_directory_id, create_scim_group_role_mapping_request)
Create a SCIM group role mapping

Creates a new SCIM group to organization role mapping for a directory. Group role mapping must be enabled on the directory.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scim_directory_id** | **String** | The ID of the directory. | [required] |
**create_scim_group_role_mapping_request** | [**CreateScimGroupRoleMappingRequest**](CreateScimGroupRoleMappingRequest.md) |  | [required] |

### Return type

[**models::ScimGroupRoleMapping**](SCIMGroupRoleMapping.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_scim_directory

> models::DeletedObject delete_scim_directory(scim_directory_id)
Delete a directory

Deletes a directory and stops provisioning for it. SCIM requests authenticated with the directory's API key are rejected afterwards.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scim_directory_id** | **String** | The ID of the directory to delete | [required] |

### Return type

[**models::DeletedObject**](DeletedObject.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_scim_group_role_mapping

> models::DeleteScimGroupRoleMapping200Response delete_scim_group_role_mapping(scim_directory_id, mapping_id)
Delete a SCIM group role mapping

Deletes a single SCIM group role mapping. Group role mapping must be enabled on the directory.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scim_directory_id** | **String** | The ID of the directory. | [required] |
**mapping_id** | **String** | The ID of the SCIM group role mapping to delete. | [required] |

### Return type

[**models::DeleteScimGroupRoleMapping200Response**](DeleteSCIMGroupRoleMapping_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scim_directory

> models::ScimDirectory get_scim_directory(scim_directory_id)
Retrieve a directory

Returns the details of a directory.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scim_directory_id** | **String** | The ID of the directory to retrieve | [required] |

### Return type

[**models::ScimDirectory**](SCIMDirectory.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_scim_directories

> models::ListScimDirectories200Response list_scim_directories(limit, offset)
List all directories

Returns a list of all directories for the instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**u32**> | Applies a limit to the number of results returned. Can be used for paginating the results together with `offset`. |  |[default to 10]
**offset** | Option<**u32**> | Skip the first `offset` results when paginating. Needs to be an integer greater or equal to zero. To be used in conjunction with `limit`. |  |[default to 0]

### Return type

[**models::ListScimDirectories200Response**](ListSCIMDirectories_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_scim_group_role_mappings

> models::ListScimGroupRoleMappings200Response list_scim_group_role_mappings(scim_directory_id)
List SCIM group role mappings

Returns the list of SCIM group to organization role mappings for a directory, ordered by precedence.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scim_directory_id** | **String** | The ID of the directory. | [required] |

### Return type

[**models::ListScimGroupRoleMappings200Response**](ListSCIMGroupRoleMappings_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_scim_group_role_mappings

> models::ListScimGroupRoleMappings200Response replace_scim_group_role_mappings(scim_directory_id, replace_scim_group_role_mappings_request)
Replace SCIM group role mappings

Replaces the entire set of SCIM group role mappings for a directory. The position of each item in the `mappings` array determines its precedence (the first item gets precedence 1). Passing an empty array removes all mappings. Group role mapping must be enabled on the directory.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scim_directory_id** | **String** | The ID of the directory. | [required] |
**replace_scim_group_role_mappings_request** | [**ReplaceScimGroupRoleMappingsRequest**](ReplaceScimGroupRoleMappingsRequest.md) |  | [required] |

### Return type

[**models::ListScimGroupRoleMappings200Response**](ListSCIMGroupRoleMappings_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rotate_scim_directory_api_key

> models::ScimDirectory rotate_scim_directory_api_key(scim_directory_id)
Rotate a directory's API key

Generates a new API key for the directory and returns it in the `api_key` field. This is the only way to obtain the key after creation, so make sure to update it in your identity provider. The previous key remains valid for a short grace period before it expires.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scim_directory_id** | **String** | The ID of the directory whose API key to rotate | [required] |

### Return type

[**models::ScimDirectory**](SCIMDirectory.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_scim_directory

> models::ScimDirectory update_scim_directory(scim_directory_id, update_scim_directory_request)
Update a directory

Updates a directory.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scim_directory_id** | **String** | The ID of the directory to update | [required] |
**update_scim_directory_request** | Option<[**UpdateScimDirectoryRequest**](UpdateScimDirectoryRequest.md)> |  |  |

### Return type

[**models::ScimDirectory**](SCIMDirectory.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

