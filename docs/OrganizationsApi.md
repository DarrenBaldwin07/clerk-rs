# \OrganizationsApi

All URIs are relative to *https://api.clerk.dev/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_organization**](OrganizationsApi.md#create_organization) | **POST** /organizations | Create an organization
[**delete_organization**](OrganizationsApi.md#delete_organization) | **DELETE** /organizations/{organization_id} | Delete an organization
[**get_organization**](OrganizationsApi.md#get_organization) | **GET** /organizations/{organization_id} | Retrieve an organization by ID or slug
[**list_organizations**](OrganizationsApi.md#list_organizations) | **GET** /organizations | Get a list of organizations for an instance
[**merge_organization_metadata**](OrganizationsApi.md#merge_organization_metadata) | **PATCH** /organizations/{organization_id}/metadata | Merge and update metadata for an organization
[**update_organization**](OrganizationsApi.md#update_organization) | **PATCH** /organizations/{organization_id} | Update an organization
[**upload_organization_logo**](OrganizationsApi.md#upload_organization_logo) | **PUT** /organizations/{organization_id}/logo | Upload a logo for the organization



## create_organization

> crate::models::Organization create_organization(create_organization_request)
Create an organization

Creates a new organization with the given name for an instance. You can specify an optional slug for the new organization. If provided, the organization slug can contain only lowercase alphanumeric characters (letters and digits) and the dash \"-\". Organization slugs must be unique for the instance. You can provide additional metadata for the organization and set any custom attribute you want. Organizations support private and public metadata. Private metadata can only be accessed from the Backend API. Public metadata can be accessed from the Backend API, and are read-only from the Frontend API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_organization_request** | Option<[**CreateOrganizationRequest**](CreateOrganizationRequest.md)> |  |  |

### Return type

[**crate::models::Organization**](Organization.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization

> crate::models::DeletedObject delete_organization(organization_id)
Delete an organization

Deletes the given organization. Please note that deleting an organization will also delete all memberships and invitations. This is not reversible.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** | The ID of the organization to delete | [required] |

### Return type

[**crate::models::DeletedObject**](DeletedObject.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization

> crate::models::Organization get_organization(organization_id)
Retrieve an organization by ID or slug

Fetches the organization whose ID or slug matches the provided `id_or_slug` URL query parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** | The ID or slug of the organization | [required] |

### Return type

[**crate::models::Organization**](Organization.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_organizations

> crate::models::Organizations list_organizations(limit, offset, include_members_count, query)
Get a list of organizations for an instance

This request returns the list of organizations for an instance. Results can be paginated using the optional `limit` and `offset` query parameters. The organizations are ordered by descending creation date. Most recent organizations will be returned first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**f32**> | Applies a limit to the number of results returned. Can be used for paginating the results together with `offset`. Must be an integer greater than zero and less than 500. By default, if not supplied, a limit of 10 is used. |  |[default to 10]
**offset** | Option<**f32**> | Skip the first `offset` results when paginating. Needs to be an integer greater or equal to zero. To be used in conjunction with `limit`. |  |[default to 0]
**include_members_count** | Option<**bool**> | Flag to denote whether the member counts of each organization should be included in the response or not. |  |
**query** | Option<**String**> | Returns organizations with ID, name, or slug that match the given query. Uses exact match for organization ID and partial match for name and slug. |  |

### Return type

[**crate::models::Organizations**](Organizations.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## merge_organization_metadata

> crate::models::Organization merge_organization_metadata(organization_id, merge_organization_metadata_request)
Merge and update metadata for an organization

Update organization metadata attributes by merging existing values with the provided parameters. Metadata values will be updated via a deep merge. Deep meaning that any nested JSON objects will be merged as well. You can remove metadata keys at any level by setting their value to `null`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** | The ID of the organization for which metadata will be merged or updated | [required] |
**merge_organization_metadata_request** | [**MergeOrganizationMetadataRequest**](MergeOrganizationMetadataRequest.md) |  | [required] |

### Return type

[**crate::models::Organization**](Organization.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_organization

> crate::models::Organization update_organization(organization_id, update_organization_request)
Update an organization

Updates an existing organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** | The ID of the organization to update | [required] |
**update_organization_request** | [**UpdateOrganizationRequest**](UpdateOrganizationRequest.md) |  | [required] |

### Return type

[**crate::models::Organization**](Organization.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_organization_logo

> crate::models::OrganizationWithLogo upload_organization_logo(organization_id, uploader_user_id, file)
Upload a logo for the organization

Set or replace an organization's logo, by uploading an image file. This endpoint uses the `multipart/form-data` request content type and accepts a file of image type. The file size cannot exceed 10MB. Only the following file content types are supported: `image/jpeg`, `image/png`, `image/gif`, `image/webp`, `image/x-icon`, `image/vnd.microsoft.icon`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** | The ID of the organization for which to upload a logo | [required] |
**uploader_user_id** | Option<**String**> |  |  |
**file** | Option<**std::path::PathBuf**> |  |  |

### Return type

[**crate::models::OrganizationWithLogo**](OrganizationWithLogo.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

