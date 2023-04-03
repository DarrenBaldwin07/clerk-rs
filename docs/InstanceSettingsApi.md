# \InstanceSettingsApi

All URIs are relative to *https://api.clerk.dev/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**update_instance**](InstanceSettingsApi.md#update_instance) | **PATCH** /instance | Update instance settings
[**update_instance_organization_settings**](InstanceSettingsApi.md#update_instance_organization_settings) | **PATCH** /instance/organization_settings | Update instance organization settings
[**update_instance_restrictions**](InstanceSettingsApi.md#update_instance_restrictions) | **PATCH** /instance/restrictions | Update instance restrictions



## update_instance

> update_instance(update_instance_request)
Update instance settings

Updates the settings of an instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_instance_request** | Option<[**UpdateInstanceRequest**](UpdateInstanceRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_instance_organization_settings

> crate::models::OrganizationSettings update_instance_organization_settings(update_instance_organization_settings_request)
Update instance organization settings

Updates the organization settings of the instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_instance_organization_settings_request** | Option<[**UpdateInstanceOrganizationSettingsRequest**](UpdateInstanceOrganizationSettingsRequest.md)> |  |  |

### Return type

[**crate::models::OrganizationSettings**](OrganizationSettings.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_instance_restrictions

> crate::models::InstanceRestrictions update_instance_restrictions(update_instance_restrictions_request)
Update instance restrictions

Updates the restriction settings of an instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_instance_restrictions_request** | Option<[**UpdateInstanceRestrictionsRequest**](UpdateInstanceRestrictionsRequest.md)> |  |  |

### Return type

[**crate::models::InstanceRestrictions**](InstanceRestrictions.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

