# \OrganizationsApi

All URIs are relative to *https://api.clerk.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**adjust_organization_billing_credit_balance**](OrganizationsApi.md#adjust_organization_billing_credit_balance) | **POST** /organizations/{organization_id}/billing/credits | Adjust an organization's credit balance
[**create_organization**](OrganizationsApi.md#create_organization) | **POST** /organizations | Create an organization
[**delete_organization**](OrganizationsApi.md#delete_organization) | **DELETE** /organizations/{organization_id} | Delete an organization
[**delete_organization_logo**](OrganizationsApi.md#delete_organization_logo) | **DELETE** /organizations/{organization_id}/logo | Delete the organization's logo.
[**get_organization**](OrganizationsApi.md#get_organization) | **GET** /organizations/{organization_id} | Retrieve an organization by ID or slug
[**get_organization_billing_credit_balance**](OrganizationsApi.md#get_organization_billing_credit_balance) | **GET** /organizations/{organization_id}/billing/credits | Retrieve an organization's credit balance
[**get_organization_billing_subscription**](OrganizationsApi.md#get_organization_billing_subscription) | **GET** /organizations/{organization_id}/billing/subscription | Retrieve an organization's billing subscription
[**list_organizations**](OrganizationsApi.md#list_organizations) | **GET** /organizations | Get a list of organizations for an instance
[**merge_organization_metadata**](OrganizationsApi.md#merge_organization_metadata) | **PATCH** /organizations/{organization_id}/metadata | Merge and update metadata for an organization
[**replace_organization_metadata**](OrganizationsApi.md#replace_organization_metadata) | **PUT** /organizations/{organization_id}/metadata | Replace metadata for an organization
[**update_organization**](OrganizationsApi.md#update_organization) | **PATCH** /organizations/{organization_id} | Update an organization
[**upload_organization_logo**](OrganizationsApi.md#upload_organization_logo) | **PUT** /organizations/{organization_id}/logo | Upload a logo for the organization



## adjust_organization_billing_credit_balance

> models::CommerceCreditLedgerResponse adjust_organization_billing_credit_balance(organization_id, adjust_credit_balance_request)
Adjust an organization's credit balance

Increases or decreases the credit balance for the specified organization. Each adjustment is recorded as a ledger entry. The idempotency_key parameter ensures that duplicate requests are safely handled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** | The ID of the organization whose credit balance to adjust | [required] |
**adjust_credit_balance_request** | [**AdjustCreditBalanceRequest**](AdjustCreditBalanceRequest.md) | Parameters for the credit balance adjustment | [required] |

### Return type

[**models::CommerceCreditLedgerResponse**](CommerceCreditLedgerResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_organization

> models::Organization create_organization(create_organization_request)
Create an organization

Creates a new organization with the given name for an instance. You can specify an optional slug for the new organization. If provided, the organization slug can contain only lowercase alphanumeric characters (letters and digits) and the dash \"-\". Organization slugs must be unique for the instance. You can provide additional metadata for the organization and set any custom attribute you want. Organizations support private and public metadata. Private metadata can only be accessed from the Backend API. Public metadata can be accessed from the Backend API, and are read-only from the Frontend API. The `created_by` user will see this as their [active organization](https://clerk.com/docs/organizations/overview#active-organization) the next time they create a session, presuming they don't explicitly set a different organization as active before then.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_organization_request** | Option<[**CreateOrganizationRequest**](CreateOrganizationRequest.md)> |  |  |

### Return type

[**models::Organization**](Organization.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization

> models::DeletedObject delete_organization(organization_id)
Delete an organization

Deletes the given organization. Please note that deleting an organization will also delete all memberships and invitations. This is not reversible.  After the organization is deleted, any user's active sessions that contain the deleted organization will be cleared.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** | The ID of the organization to delete | [required] |

### Return type

[**models::DeletedObject**](DeletedObject.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization_logo

> models::Organization delete_organization_logo(organization_id)
Delete the organization's logo.

Delete the organization's logo.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** | The ID of the organization for which the logo will be deleted. | [required] |

### Return type

[**models::Organization**](Organization.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization

> models::Organization get_organization(organization_id, include_members_count, include_missing_member_with_elevated_permissions)
Retrieve an organization by ID or slug

Fetches the organization whose ID or slug matches the provided `id_or_slug` URL query parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** | The ID or slug of the organization | [required] |
**include_members_count** | Option<**bool**> | Flag to denote whether or not the organization's members count should be included in the response. |  |
**include_missing_member_with_elevated_permissions** | Option<**bool**> | Flag to denote whether or not to include a member with elevated permissions who is not currently a member of the organization. |  |

### Return type

[**models::Organization**](Organization.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_billing_credit_balance

> models::CommerceCreditBalanceResponse get_organization_billing_credit_balance(organization_id)
Retrieve an organization's credit balance

Retrieves the current credit balance for the specified organization. Credits can be applied during checkout to reduce the charge or automatically applied to upcoming recurring charges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** | The ID of the organization whose credit balance to retrieve | [required] |

### Return type

[**models::CommerceCreditBalanceResponse**](CommerceCreditBalanceResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_billing_subscription

> models::CommerceSubscription get_organization_billing_subscription(organization_id)
Retrieve an organization's billing subscription

Retrieves the billing subscription for the specified organization. This includes subscription details, active plans, billing information, and payment status. The subscription contains subscription items which represent the individual plans the organization is subscribed to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** | The ID of the organization whose subscription to retrieve | [required] |

### Return type

[**models::CommerceSubscription**](CommerceSubscription.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_organizations

> models::Organizations list_organizations(include_members_count, include_missing_member_with_elevated_permissions, query, user_id, organization_id, order_by, limit, offset)
Get a list of organizations for an instance

This request returns the list of organizations for an instance. Results can be paginated using the optional `limit` and `offset` query parameters. The organizations are ordered by descending creation date. Most recent organizations will be returned first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_members_count** | Option<**bool**> | Flag to denote whether the member counts of each organization should be included in the response or not. |  |
**include_missing_member_with_elevated_permissions** | Option<**bool**> | Flag to denote whether or not to include a member with elevated permissions who is not currently a member of the organization. |  |
**query** | Option<**String**> | Returns organizations with ID, name, or slug that match the given query. Uses exact match for organization ID and partial match for name and slug. |  |
**user_id** | Option<[**Vec<String>**](String.md)> | Returns organizations that include any of the specified user IDs as members. Any user IDs not found are ignored. For each user ID, the `+` and `-` can be prepended to the ID, which denote whether the respective organization should be included or excluded from the result set. |  |
**organization_id** | Option<[**Vec<String>**](String.md)> | Returns organizations with the organization IDs specified. Any organization IDs not found are ignored. For each organization ID, the `+` and `-` can be prepended to the ID, which denote whether the respective organization should be included or excluded from the result set. Accepts up to 100 organization IDs. Example: ?organization_id=+org_1&organization_id=-org_2 |  |
**order_by** | Option<**String**> | Allows to return organizations in a particular order. At the moment, you can order the returned organizations either by their `name`, `created_at` or `members_count`. In order to specify the direction, you can use the `+/-` symbols prepended in the property to order by. For example, if you want organizations to be returned in descending order according to their `created_at` property, you can use `-created_at`. If you don't use `+` or `-`, then `+` is implied. Defaults to `-created_at`. |  |[default to -created_at]
**limit** | Option<**u32**> | Applies a limit to the number of results returned. Can be used for paginating the results together with `offset`. |  |[default to 10]
**offset** | Option<**u32**> | Skip the first `offset` results when paginating. Needs to be an integer greater or equal to zero. To be used in conjunction with `limit`. |  |[default to 0]

### Return type

[**models::Organizations**](Organizations.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## merge_organization_metadata

> models::Organization merge_organization_metadata(organization_id, merge_organization_metadata_request)
Merge and update metadata for an organization

Update organization metadata attributes by merging existing values with the provided parameters. Metadata values will be updated via a deep merge. Deep meaning that any nested JSON objects will be merged as well. You can remove metadata keys at any level by setting their value to `null`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** | The ID of the organization for which metadata will be merged or updated | [required] |
**merge_organization_metadata_request** | [**MergeOrganizationMetadataRequest**](MergeOrganizationMetadataRequest.md) |  | [required] |

### Return type

[**models::Organization**](Organization.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_organization_metadata

> models::Organization replace_organization_metadata(organization_id, replace_organization_metadata_request)
Replace metadata for an organization

Replace an organization's metadata attributes with the provided values. Unlike `PATCH /v1/organizations/{organization_id}/metadata` (merge semantics), this endpoint replaces the supplied metadata fields entirely — the prior contents of each supplied field are discarded. Fields omitted from the request body are left unchanged. Prefer the `PATCH` endpoint for partial updates. Use `PUT` only when you explicitly intend to overwrite a metadata field wholesale.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** | The ID of the organization whose metadata will be replaced | [required] |
**replace_organization_metadata_request** | [**ReplaceOrganizationMetadataRequest**](ReplaceOrganizationMetadataRequest.md) |  | [required] |

### Return type

[**models::Organization**](Organization.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_organization

> models::Organization update_organization(organization_id, update_organization_request)
Update an organization

Updates an existing organization.  As of API version 2026-05-12, this endpoint no longer accepts `public_metadata` or `private_metadata`. Use `PATCH /v1/organizations/{organization_id}/metadata` to merge updates into existing metadata, or `PUT /v1/organizations/{organization_id}/metadata` to replace a metadata field entirely.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** | The ID of the organization to update | [required] |
**update_organization_request** | [**UpdateOrganizationRequest**](UpdateOrganizationRequest.md) |  | [required] |

### Return type

[**models::Organization**](Organization.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_organization_logo

> models::OrganizationWithLogo upload_organization_logo(organization_id, file, uploader_user_id)
Upload a logo for the organization

Set or replace an organization's logo, by uploading an image file. This endpoint uses the `multipart/form-data` request content type and accepts a file of image type. The file size cannot exceed 10MB. Only the following file content types are supported: `image/jpeg`, `image/png`, `image/gif`, `image/webp`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** | The ID of the organization for which to upload a logo | [required] |
**file** | **std::path::PathBuf** |  | [required] |
**uploader_user_id** | Option<**String**> | The ID of the user that will be credited with the image upload. |  |

### Return type

[**models::OrganizationWithLogo**](OrganizationWithLogo.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

