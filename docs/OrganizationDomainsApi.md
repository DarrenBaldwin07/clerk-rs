# \OrganizationDomainsApi

All URIs are relative to *https://api.clerk.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_organization_domain**](OrganizationDomainsApi.md#create_organization_domain) | **POST** /organizations/{organization_id}/domains | Create a new organization domain.
[**delete_organization_domain**](OrganizationDomainsApi.md#delete_organization_domain) | **DELETE** /organizations/{organization_id}/domains/{domain_id} | Remove a domain from an organization.
[**list_all_organization_domains**](OrganizationDomainsApi.md#list_all_organization_domains) | **GET** /organization_domains | List all organization domains
[**list_organization_domains**](OrganizationDomainsApi.md#list_organization_domains) | **GET** /organizations/{organization_id}/domains | Get a list of all domains of an organization.
[**update_organization_domain**](OrganizationDomainsApi.md#update_organization_domain) | **PATCH** /organizations/{organization_id}/domains/{domain_id} | Update an organization domain.
[**verify_organization_domain_ownership**](OrganizationDomainsApi.md#verify_organization_domain_ownership) | **POST** /organizations/{organization_id}/domains/{domain_id}/verify_ownership | Mark an organization domain's ownership as verified



## create_organization_domain

> models::OrganizationDomain create_organization_domain(organization_id, create_organization_domain_request)
Create a new organization domain.

Creates a new organization domain. By default the domain is verified, but can be optionally set to unverified.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** | The ID of the organization where the new domain will be created. | [required] |
**create_organization_domain_request** | [**CreateOrganizationDomainRequest**](CreateOrganizationDomainRequest.md) |  | [required] |

### Return type

[**models::OrganizationDomain**](OrganizationDomain.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization_domain

> models::DeletedObject delete_organization_domain(organization_id, domain_id)
Remove a domain from an organization.

Removes the given domain from the organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** | The ID of the organization to which the domain belongs | [required] |
**domain_id** | **String** | The ID of the domain | [required] |

### Return type

[**models::DeletedObject**](DeletedObject.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_all_organization_domains

> models::OrganizationDomains list_all_organization_domains(organization_id, verified, enrollment_mode, query, domains, order_by, offset, limit)
List all organization domains

Retrieves a list of all organization domains within the current instance. This endpoint can be used to list all domains across all organizations or filter domains by organization, verification status, enrollment mode, or search query.  The response includes pagination information and details about each domain including its verification status, enrollment mode, and associated counts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<**String**> | The ID of the organization to filter domains by |  |
**verified** | Option<**String**> | Filter by verification status |  |
**enrollment_mode** | Option<[**Vec<String>**](String.md)> | Filter by enrollment mode. Prefix a value with `-` to exclude it instead (e.g. `-enterprise_sso`). Include and exclude filters can be combined and compose as AND. |  |
**query** | Option<**String**> | Search domains by name or organization ID. If the query starts with \"org_\", it will search by exact organization ID match. Otherwise, it performs a case-insensitive partial match on the domain name.  Note: An empty string or whitespace-only value is not allowed and will result in a validation error.  |  |
**domains** | Option<[**Vec<String>**](String.md)> | Filter by exact domain names. Accepts multiple values (e.g. domains=example.com&domains=test.org). |  |
**order_by** | Option<**String**> | Allows to return organization domains in a particular order. At the moment, you can order the returned domains by their `name` or `created_at`. In order to specify the direction, you can use the `+/-` symbols prepended to the property to order by. For example, if you want domains to be returned in descending order according to their `created_at` property, you can use `-created_at`. If you don't use `+` or `-`, then `+` is implied. Defaults to `-created_at`.  |  |[default to -created_at]
**offset** | Option<**u32**> | Skip the first `offset` results when paginating. Needs to be an integer greater or equal to zero. To be used in conjunction with `limit`. |  |[default to 0]
**limit** | Option<**u32**> | Applies a limit to the number of results returned. Can be used for paginating the results together with `offset`. |  |[default to 10]

### Return type

[**models::OrganizationDomains**](OrganizationDomains.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_organization_domains

> models::OrganizationDomains list_organization_domains(organization_id, verified, enrollment_mode, limit, offset)
Get a list of all domains of an organization.

Get a list of all domains of an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** | The organization ID. | [required] |
**verified** | Option<**String**> | Filter domains by their verification status. `true` or `false` |  |
**enrollment_mode** | Option<**String**> | Filter domains by their enrollment mode. Accepts `automatic_invitation`, `automatic_suggestion`, `manual_invitation` or `enterprise_sso`. Prefix a value with `-` to exclude it instead (e.g. `-enterprise_sso`). Include and exclude filters can be combined and compose as AND. |  |
**limit** | Option<**u32**> | Applies a limit to the number of results returned. Can be used for paginating the results together with `offset`. |  |[default to 10]
**offset** | Option<**u32**> | Skip the first `offset` results when paginating. Needs to be an integer greater or equal to zero. To be used in conjunction with `limit`. |  |[default to 0]

### Return type

[**models::OrganizationDomains**](OrganizationDomains.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_organization_domain

> models::OrganizationDomain update_organization_domain(organization_id, domain_id, update_organization_domain_request)
Update an organization domain.

Updates the properties of an existing organization domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** | The ID of the organization to which the domain belongs | [required] |
**domain_id** | **String** | The ID of the domain | [required] |
**update_organization_domain_request** | [**UpdateOrganizationDomainRequest**](UpdateOrganizationDomainRequest.md) |  | [required] |

### Return type

[**models::OrganizationDomain**](OrganizationDomain.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_organization_domain_ownership

> models::OrganizationDomain verify_organization_domain_ownership(organization_id, domain_id)
Mark an organization domain's ownership as verified

Flips the organization domain's ownership state to verified via the manual override path, bypassing the self-serve TXT DNS challenge. The domain row records strategy=`manual_override` and an `organization_domain.ownership_verified` audit event is emitted with the same strategy.  Idempotent: re-calling on an already-verified domain returns the current ownership state without re-emitting the audit event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** | The ID of the organization to which the domain belongs | [required] |
**domain_id** | **String** | The ID of the domain | [required] |

### Return type

[**models::OrganizationDomain**](OrganizationDomain.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

