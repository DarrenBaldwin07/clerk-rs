# \BetaFeaturesApi

All URIs are relative to *https://api.clerk.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**change_production_instance_domain**](BetaFeaturesApi.md#change_production_instance_domain) | **POST** /instance/change_domain | Update production instance domain
[**update_instance_auth_config**](BetaFeaturesApi.md#update_instance_auth_config) | **PATCH** /beta_features/instance_settings | Update instance settings
[**update_production_instance_domain**](BetaFeaturesApi.md#update_production_instance_domain) | **PUT** /beta_features/domain | Update production instance domain



## change_production_instance_domain

> change_production_instance_domain(update_production_instance_domain_request)
Update production instance domain

Change the domain of a production instance.  Changing the domain requires updating the [DNS records](https://clerk.com/docs/deployments/overview#dns-records) accordingly, deploying new [SSL certificates](https://clerk.com/docs/deployments/overview#deploy), updating your Social Connection's redirect URLs and setting the new keys in your code.  WARNING: Changing your domain will invalidate all current user sessions (i.e. users will be logged out). Also, while your application is being deployed, a small downtime is expected to occur.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_production_instance_domain_request** | Option<[**UpdateProductionInstanceDomainRequest**](UpdateProductionInstanceDomainRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_instance_auth_config

> crate::models::UpdateInstanceAuthConfig200Response update_instance_auth_config(update_instance_auth_config_request)
Update instance settings

Updates the settings of an instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_instance_auth_config_request** | Option<[**UpdateInstanceAuthConfigRequest**](UpdateInstanceAuthConfigRequest.md)> |  |  |

### Return type

[**crate::models::UpdateInstanceAuthConfig200Response**](UpdateInstanceAuthConfig_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_production_instance_domain

> update_production_instance_domain(update_production_instance_domain_request)
Update production instance domain

Change the domain of a production instance.  Changing the domain requires updating the [DNS records](https://clerk.com/docs/deployments/overview#dns-records) accordingly, deploying new [SSL certificates](https://clerk.com/docs/deployments/overview#deploy), updating your Social Connection's redirect URLs and setting the new keys in your code.  WARNING: Changing your domain will invalidate all current user sessions (i.e. users will be logged out). Also, while your application is being deployed, a small downtime is expected to occur.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_production_instance_domain_request** | Option<[**UpdateProductionInstanceDomainRequest**](UpdateProductionInstanceDomainRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

