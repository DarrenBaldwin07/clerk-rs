# \DirectoriesApi

All URIs are relative to *https://api.clerk.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_directory**](DirectoriesApi.md#create_directory) | **POST** /directories | Create a directory
[**create_directory_group_role_mapping**](DirectoriesApi.md#create_directory_group_role_mapping) | **POST** /directories/{directory_id}/group_role_mappings | Create a directory group role mapping
[**delete_directory**](DirectoriesApi.md#delete_directory) | **DELETE** /directories/{directory_id} | Delete a directory
[**delete_directory_group_role_mapping**](DirectoriesApi.md#delete_directory_group_role_mapping) | **DELETE** /directories/{directory_id}/group_role_mappings/{mapping_id} | Delete a directory group role mapping
[**get_directory**](DirectoriesApi.md#get_directory) | **GET** /directories/{directory_id} | Retrieve a directory
[**list_directories**](DirectoriesApi.md#list_directories) | **GET** /directories | List all directories
[**list_directory_group_role_mappings**](DirectoriesApi.md#list_directory_group_role_mappings) | **GET** /directories/{directory_id}/group_role_mappings | List directory group role mappings
[**replace_directory_group_role_mappings**](DirectoriesApi.md#replace_directory_group_role_mappings) | **PUT** /directories/{directory_id}/group_role_mappings | Replace directory group role mappings
[**rotate_directory_api_key**](DirectoriesApi.md#rotate_directory_api_key) | **POST** /directories/{directory_id}/rotate_api_key | Rotate a directory's API key
[**update_directory**](DirectoriesApi.md#update_directory) | **PATCH** /directories/{directory_id} | Update a directory



## create_directory

> models::Directory create_directory(create_directory_request)
Create a directory

Create a new directory for the instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_directory_request** | Option<[**CreateDirectoryRequest**](CreateDirectoryRequest.md)> |  |  |

### Return type

[**models::Directory**](Directory.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_directory_group_role_mapping

> models::DirectoryGroupRoleMapping create_directory_group_role_mapping(directory_id, create_directory_group_role_mapping_request)
Create a directory group role mapping

Creates a new directory group to organization role mapping for a directory. Group role mapping must be enabled on the directory.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**directory_id** | **String** | The ID of the directory. | [required] |
**create_directory_group_role_mapping_request** | [**CreateDirectoryGroupRoleMappingRequest**](CreateDirectoryGroupRoleMappingRequest.md) |  | [required] |

### Return type

[**models::DirectoryGroupRoleMapping**](DirectoryGroupRoleMapping.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_directory

> models::DeletedObject delete_directory(directory_id)
Delete a directory

Deletes a directory and stops provisioning for it. Provisioning requests authenticated with the directory's API key are rejected afterwards.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**directory_id** | **String** | The ID of the directory to delete | [required] |

### Return type

[**models::DeletedObject**](DeletedObject.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_directory_group_role_mapping

> models::DeleteDirectoryGroupRoleMapping200Response delete_directory_group_role_mapping(directory_id, mapping_id)
Delete a directory group role mapping

Deletes a single directory group role mapping. Group role mapping must be enabled on the directory.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**directory_id** | **String** | The ID of the directory. | [required] |
**mapping_id** | **String** | The ID of the directory group role mapping to delete. | [required] |

### Return type

[**models::DeleteDirectoryGroupRoleMapping200Response**](DeleteDirectoryGroupRoleMapping_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_directory

> models::Directory get_directory(directory_id)
Retrieve a directory

Returns the details of a directory.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**directory_id** | **String** | The ID of the directory to retrieve | [required] |

### Return type

[**models::Directory**](Directory.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_directories

> models::ListDirectories200Response list_directories(limit, offset)
List all directories

Returns a list of all directories for the instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**u32**> | Applies a limit to the number of results returned. Can be used for paginating the results together with `offset`. |  |[default to 10]
**offset** | Option<**u32**> | Skip the first `offset` results when paginating. Needs to be an integer greater or equal to zero. To be used in conjunction with `limit`. |  |[default to 0]

### Return type

[**models::ListDirectories200Response**](ListDirectories_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_directory_group_role_mappings

> models::ListDirectoryGroupRoleMappings200Response list_directory_group_role_mappings(directory_id)
List directory group role mappings

Returns the list of directory group to organization role mappings for a directory, ordered by precedence.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**directory_id** | **String** | The ID of the directory. | [required] |

### Return type

[**models::ListDirectoryGroupRoleMappings200Response**](ListDirectoryGroupRoleMappings_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_directory_group_role_mappings

> models::ListDirectoryGroupRoleMappings200Response replace_directory_group_role_mappings(directory_id, replace_directory_group_role_mappings_request)
Replace directory group role mappings

Replaces the entire set of directory group role mappings for a directory. The position of each item in the `mappings` array determines its precedence (the first item gets precedence 1). Passing an empty array removes all mappings. Group role mapping must be enabled on the directory.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**directory_id** | **String** | The ID of the directory. | [required] |
**replace_directory_group_role_mappings_request** | [**ReplaceDirectoryGroupRoleMappingsRequest**](ReplaceDirectoryGroupRoleMappingsRequest.md) |  | [required] |

### Return type

[**models::ListDirectoryGroupRoleMappings200Response**](ListDirectoryGroupRoleMappings_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rotate_directory_api_key

> models::Directory rotate_directory_api_key(directory_id)
Rotate a directory's API key

Generates a new API key for the directory and returns it in the `api_key` field. This is the only way to obtain the key after creation, so make sure to update it in your identity provider. The previous key remains valid for a short grace period before it expires.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**directory_id** | **String** | The ID of the directory whose API key to rotate | [required] |

### Return type

[**models::Directory**](Directory.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_directory

> models::Directory update_directory(directory_id, update_directory_request)
Update a directory

Updates a directory.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**directory_id** | **String** | The ID of the directory to update | [required] |
**update_directory_request** | Option<[**UpdateDirectoryRequest**](UpdateDirectoryRequest.md)> |  |  |

### Return type

[**models::Directory**](Directory.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

