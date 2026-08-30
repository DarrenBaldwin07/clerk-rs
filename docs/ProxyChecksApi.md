# \ProxyChecksApi

All URIs are relative to *https://api.clerk.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**verify_domain_proxy**](ProxyChecksApi.md#verify_domain_proxy) | **POST** /proxy_checks | Verify the proxy configuration for your domain



## verify_domain_proxy

> models::ProxyCheck verify_domain_proxy(verify_domain_proxy_request)
Verify the proxy configuration for your domain

This endpoint can be used to validate that a proxy-enabled domain is operational. It tries to verify that the proxy URL provided in the parameters maps to a functional proxy that can reach the Clerk Frontend API.  You can use this endpoint before you set a proxy URL for a domain. This way you can ensure that switching to proxy-based configuration will not lead to downtime for your instance.  The `proxy_url` parameter allows for testing proxy configurations for domains that don't have a proxy URL yet, or operate on a different proxy URL than the one provided. It can also be used to re-validate a domain that is already configured to work with a proxy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verify_domain_proxy_request** | Option<[**VerifyDomainProxyRequest**](VerifyDomainProxyRequest.md)> |  |  |

### Return type

[**models::ProxyCheck**](ProxyCheck.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

