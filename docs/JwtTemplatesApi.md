# \JwtTemplatesApi

All URIs are relative to *https://api.clerk.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_jwt_template**](JwtTemplatesApi.md#create_jwt_template) | **POST** /jwt_templates | Create a JWT template
[**delete_jwt_template**](JwtTemplatesApi.md#delete_jwt_template) | **DELETE** /jwt_templates/{template_id} | Delete a Template
[**get_jwt_template**](JwtTemplatesApi.md#get_jwt_template) | **GET** /jwt_templates/{template_id} | Retrieve a template
[**list_jwt_templates**](JwtTemplatesApi.md#list_jwt_templates) | **GET** /jwt_templates | List all templates
[**update_jwt_template**](JwtTemplatesApi.md#update_jwt_template) | **PATCH** /jwt_templates/{template_id} | Update a JWT template



## create_jwt_template

> models::JwtTemplate create_jwt_template(create_jwt_template_request)
Create a JWT template

Create a new JWT template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_jwt_template_request** | Option<[**CreateJwtTemplateRequest**](CreateJwtTemplateRequest.md)> |  |  |

### Return type

[**models::JwtTemplate**](JWTTemplate.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_jwt_template

> models::DeletedObject delete_jwt_template(template_id)
Delete a Template



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | JWT Template ID | [required] |

### Return type

[**models::DeletedObject**](DeletedObject.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_jwt_template

> models::JwtTemplate get_jwt_template(template_id)
Retrieve a template

Retrieve the details of a given JWT template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | JWT Template ID | [required] |

### Return type

[**models::JwtTemplate**](JWTTemplate.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_jwt_templates

> Vec<models::JwtTemplate> list_jwt_templates(paginated, limit, offset)
List all templates

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**paginated** | Option<**bool**> | Whether to paginate the results. If true, the results will be paginated. If false, the results will not be paginated. |  |
**limit** | Option<**u32**> | Applies a limit to the number of results returned. Can be used for paginating the results together with `offset`. |  |[default to 10]
**offset** | Option<**u32**> | Skip the first `offset` results when paginating. Needs to be an integer greater or equal to zero. To be used in conjunction with `limit`. |  |[default to 0]

### Return type

[**Vec<models::JwtTemplate>**](JWTTemplate.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_jwt_template

> models::JwtTemplate update_jwt_template(template_id, create_jwt_template_request)
Update a JWT template

Updates an existing JWT template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | The ID of the JWT template to update | [required] |
**create_jwt_template_request** | Option<[**CreateJwtTemplateRequest**](CreateJwtTemplateRequest.md)> |  |  |

### Return type

[**models::JwtTemplate**](JWTTemplate.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

