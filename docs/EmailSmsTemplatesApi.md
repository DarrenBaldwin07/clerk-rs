# \EmailSmsTemplatesApi

All URIs are relative to *https://api.clerk.dev/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_template**](EmailSmsTemplatesApi.md#get_template) | **GET** /templates/{template_type}/{slug} | Retrieve a template
[**get_template_list**](EmailSmsTemplatesApi.md#get_template_list) | **GET** /templates/{template_type} | List all templates
[**preview_template**](EmailSmsTemplatesApi.md#preview_template) | **POST** /templates/{template_type}/{slug}/preview | Preview changes to a template
[**revert_template**](EmailSmsTemplatesApi.md#revert_template) | **POST** /templates/{template_type}/{slug}/revert | Revert a template
[**upsert_template**](EmailSmsTemplatesApi.md#upsert_template) | **PUT** /templates/{template_type}/{slug} | Update a template for a given type and slug



## get_template

> crate::models::Template get_template(template_type, slug)
Retrieve a template

Returns the details of a template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_type** | **String** | The type of templates to retrieve (email or SMS) | [required] |
**slug** | **String** | The slug (i.e. machine-friendly name) of the template to retrieve | [required] |

### Return type

[**crate::models::Template**](Template.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_template_list

> Vec<crate::models::Template> get_template_list(template_type)
List all templates

Returns a list of all templates. The templates are returned sorted by position.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_type** | **String** | The type of templates to list (email or SMS) | [required] |

### Return type

[**Vec<crate::models::Template>**](Template.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## preview_template

> serde_json::Value preview_template(template_type, slug, preview_template_request)
Preview changes to a template

Returns a preview of a template for a given template_type, slug and body

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_type** | **String** | The type of template to preview | [required] |
**slug** | **String** | The slug of the template to preview | [required] |
**preview_template_request** | Option<[**PreviewTemplateRequest**](PreviewTemplateRequest.md)> | Required parameters |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revert_template

> crate::models::Template revert_template(template_type, slug)
Revert a template

Reverts an updated template to its default state

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_type** | **String** | The type of template to revert | [required] |
**slug** | **String** | The slug of the template to revert | [required] |

### Return type

[**crate::models::Template**](Template.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upsert_template

> crate::models::Template upsert_template(template_type, slug, upsert_template_request)
Update a template for a given type and slug

Updates the existing template of the given type and slug

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_type** | **String** | The type of template to update | [required] |
**slug** | **String** | The slug of the template to update | [required] |
**upsert_template_request** | Option<[**UpsertTemplateRequest**](UpsertTemplateRequest.md)> |  |  |

### Return type

[**crate::models::Template**](Template.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

