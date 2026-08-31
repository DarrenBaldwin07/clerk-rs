# \InstanceSettingsApi

All URIs are relative to *https://api.clerk.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_instance**](InstanceSettingsApi.md#get_instance) | **GET** /instance | Fetch the current instance
[**get_instance_communication**](InstanceSettingsApi.md#get_instance_communication) | **GET** /instance/communication | Get instance communication settings
[**get_instance_o_auth_application_settings**](InstanceSettingsApi.md#get_instance_o_auth_application_settings) | **GET** /instance/oauth_application_settings | Get OAuth application settings
[**get_instance_organization_settings**](InstanceSettingsApi.md#get_instance_organization_settings) | **GET** /instance/organization_settings | Get instance organization settings
[**get_instance_protect**](InstanceSettingsApi.md#get_instance_protect) | **GET** /instance/protect | Get instance protect settings
[**update_instance**](InstanceSettingsApi.md#update_instance) | **PATCH** /instance | Update instance settings
[**update_instance_communication**](InstanceSettingsApi.md#update_instance_communication) | **PATCH** /instance/communication | Update instance communication settings
[**update_instance_o_auth_application_settings**](InstanceSettingsApi.md#update_instance_o_auth_application_settings) | **PATCH** /instance/oauth_application_settings | Update OAuth application settings
[**update_instance_organization_settings**](InstanceSettingsApi.md#update_instance_organization_settings) | **PATCH** /instance/organization_settings | Update instance organization settings
[**update_instance_protect**](InstanceSettingsApi.md#update_instance_protect) | **PATCH** /instance/protect | Update instance protect settings
[**update_instance_restrictions**](InstanceSettingsApi.md#update_instance_restrictions) | **PATCH** /instance/restrictions | Update instance restrictions



## get_instance

> models::Instance get_instance()
Fetch the current instance

Fetches the current instance

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Instance**](Instance.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instance_communication

> models::InstanceCommunication get_instance_communication()
Get instance communication settings

Retrieves the per-instance SMS communication settings, including the SMS country blocklist.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::InstanceCommunication**](InstanceCommunication.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instance_o_auth_application_settings

> models::OAuthApplicationSettings get_instance_o_auth_application_settings()
Get OAuth application settings

Retrieves the settings for OAuth applications for the instance (dynamic client registration, JWT access tokens, etc.).

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::OAuthApplicationSettings**](OAuthApplicationSettings.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instance_organization_settings

> models::OrganizationSettings get_instance_organization_settings()
Get instance organization settings

Retrieves the organization settings of the instance

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::OrganizationSettings**](OrganizationSettings.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instance_protect

> models::InstanceProtect get_instance_protect()
Get instance protect settings

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::InstanceProtect**](InstanceProtect.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## update_instance_communication

> models::InstanceCommunication update_instance_communication(update_instance_communication_request)
Update instance communication settings

Replaces the SMS country blocklist for this instance. Pass the full set of ISO 3166-1 alpha-2 country codes that should be blocked; codes that aren't recognized as SMS-tier countries are silently dropped from the persisted list. Omitting `blocked_country_codes` is a no-op.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_instance_communication_request** | Option<[**UpdateInstanceCommunicationRequest**](UpdateInstanceCommunicationRequest.md)> |  |  |

### Return type

[**models::InstanceCommunication**](InstanceCommunication.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_instance_o_auth_application_settings

> models::OAuthApplicationSettings update_instance_o_auth_application_settings(update_instance_o_auth_application_settings_request)
Update OAuth application settings

Updates the OAuth application settings for the instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_instance_o_auth_application_settings_request** | Option<[**UpdateInstanceOAuthApplicationSettingsRequest**](UpdateInstanceOAuthApplicationSettingsRequest.md)> |  |  |

### Return type

[**models::OAuthApplicationSettings**](OAuthApplicationSettings.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_instance_organization_settings

> models::OrganizationSettings update_instance_organization_settings(update_instance_organization_settings_request)
Update instance organization settings

Updates the organization settings of the instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_instance_organization_settings_request** | Option<[**UpdateInstanceOrganizationSettingsRequest**](UpdateInstanceOrganizationSettingsRequest.md)> |  |  |

### Return type

[**models::OrganizationSettings**](OrganizationSettings.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_instance_protect

> models::InstanceProtect update_instance_protect(update_instance_protect_request)
Update instance protect settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_instance_protect_request** | Option<[**UpdateInstanceProtectRequest**](UpdateInstanceProtectRequest.md)> |  |  |

### Return type

[**models::InstanceProtect**](InstanceProtect.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_instance_restrictions

> models::InstanceRestrictions update_instance_restrictions(update_instance_restrictions_request)
Update instance restrictions

Updates the restriction settings of an instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_instance_restrictions_request** | Option<[**UpdateInstanceRestrictionsRequest**](UpdateInstanceRestrictionsRequest.md)> |  |  |

### Return type

[**models::InstanceRestrictions**](InstanceRestrictions.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

