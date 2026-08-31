# \EmailSmsTemplatesApi

All URIs are relative to *https://api.clerk.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_template**](EmailSmsTemplatesApi.md#get_template) | **GET** /templates/{template_type}/{slug} | Retrieve a template
[**get_template_list**](EmailSmsTemplatesApi.md#get_template_list) | **GET** /templates/{template_type} | List all templates
[**preview_template**](EmailSmsTemplatesApi.md#preview_template) | **POST** /templates/{template_type}/{slug}/preview | Preview changes to a template
[**revert_template**](EmailSmsTemplatesApi.md#revert_template) | **POST** /templates/{template_type}/{slug}/revert | Revert a template
[**toggle_template_delivery**](EmailSmsTemplatesApi.md#toggle_template_delivery) | **POST** /templates/{template_type}/{slug}/toggle_delivery | Toggle the delivery by Clerk for a template of a given type and slug
[**upsert_template**](EmailSmsTemplatesApi.md#upsert_template) | **PUT** /templates/{template_type}/{slug} | Update a template for a given type and slug



## get_template

> models::Template get_template(template_type, slug)
Retrieve a template

Returns the details of a template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_type** | **String** | The type of templates to retrieve (email or SMS) | [required] |
**slug** | **String** | The slug (i.e. machine-friendly name) of the template to retrieve | [required] |

### Return type

[**models::Template**](Template.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_template_list

> Vec<models::Template> get_template_list(template_type, paginated, limit, offset)
List all templates

Returns a list of all templates. The templates are returned sorted by position.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_type** | **String** | The type of templates to list (email or SMS) | [required] |
**paginated** | Option<**bool**> | Whether to paginate the results. If true, the results will be paginated. If false, the results will not be paginated. |  |
**limit** | Option<**u32**> | Applies a limit to the number of results returned. Can be used for paginating the results together with `offset`. |  |[default to 10]
**offset** | Option<**u32**> | Skip the first `offset` results when paginating. Needs to be an integer greater or equal to zero. To be used in conjunction with `limit`. |  |[default to 0]

### Return type

[**Vec<models::Template>**](Template.md)

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

> models::Template revert_template(template_type, slug)
Revert a template

Reverts an updated template to its default state

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_type** | **String** | The type of template to revert | [required] |
**slug** | **String** | The slug of the template to revert | [required] |

### Return type

[**models::Template**](Template.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## toggle_template_delivery

> models::Template toggle_template_delivery(template_type, slug, toggle_template_delivery_request)
Toggle the delivery by Clerk for a template of a given type and slug

Toggles the delivery by Clerk for a template of a given type and slug. If disabled, Clerk will not deliver the resulting email or SMS. The app developer will need to listen to the `email.created` or `sms.created` webhooks in order to handle delivery themselves.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_type** | **String** | The type of template to toggle delivery for | [required] |
**slug** | **String** | The slug of the template for which to toggle delivery | [required] |
**toggle_template_delivery_request** | Option<[**ToggleTemplateDeliveryRequest**](ToggleTemplateDeliveryRequest.md)> |  |  |

### Return type

[**models::Template**](Template.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upsert_template

> models::Template upsert_template(template_type, slug, upsert_template_request)
Update a template for a given type and slug

Updates the existing template of the given type and slug

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_type** | **String** | The type of template to update | [required] |
**slug** | **String** | The slug of the template to update | [required] |
**upsert_template_request** | Option<[**UpsertTemplateRequest**](UpsertTemplateRequest.md)> |  |  |

### Return type

[**models::Template**](Template.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

