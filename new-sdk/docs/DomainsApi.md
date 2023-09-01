# \DomainsApi

All URIs are relative to *https://api.clerk.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_domain**](DomainsApi.md#add_domain) | **POST** /domains | Add a domain
[**delete_domain**](DomainsApi.md#delete_domain) | **DELETE** /domains/{domain_id} | Delete a satellite domain
[**list_domains**](DomainsApi.md#list_domains) | **GET** /domains | List all instance domains
[**update_domain**](DomainsApi.md#update_domain) | **PATCH** /domains/{domain_id} | Update a domain



## add_domain

> crate::models::Domain add_domain(add_domain_request)
Add a domain

Add a new domain for your instance. Useful in the case of multi-domain instances, allows adding satellite domains to an instance. The new domain must have a `name`. The domain name can contain the port for development instances, like `localhost:3000`. At the moment, instances can have only one primary domain, so the `is_satellite` parameter must be set to `true`. If you're planning to configure the new satellite domain to run behind a proxy, pass the `proxy_url` parameter accordingly.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_domain_request** | Option<[**AddDomainRequest**](AddDomainRequest.md)> |  |  |

### Return type

[**crate::models::Domain**](Domain.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_domain

> crate::models::DeletedObject delete_domain(domain_id)
Delete a satellite domain

Deletes a satellite domain for the instance. It is currently not possible to delete the instance's primary domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** | The ID of the domain that will be deleted. Must be a satellite domain. | [required] |

### Return type

[**crate::models::DeletedObject**](DeletedObject.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_domains

> crate::models::Domains list_domains()
List all instance domains

Use this endpoint to get a list of all domains for an instance. The response will contain the primary domain for the instance and any satellite domains. Each domain in the response contains information about the URLs where Clerk operates and the required CNAME targets.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Domains**](Domains.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_domain

> crate::models::Domain update_domain(domain_id, update_domain_request)
Update a domain

The `proxy_url` can be updated only for production instances. Update one of the instance's domains. Both primary and satellite domains can be updated. If you choose to use Clerk via proxy, use this endpoint to specify the `proxy_url`. Whenever you decide you'd rather switch to DNS setup for Clerk, simply set `proxy_url` to `null` for the domain. When you update a production instance's primary domain name, you have to make sure that you've completed all the necessary setup steps for DNS and emails to work. Expect downtime otherwise. Updating a primary domain's name will also update the instance's home origin, affecting the default application paths.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** | The ID of the domain that will be updated. | [required] |
**update_domain_request** | [**UpdateDomainRequest**](UpdateDomainRequest.md) |  | [required] |

### Return type

[**crate::models::Domain**](Domain.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

