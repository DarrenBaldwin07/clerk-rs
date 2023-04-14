# \JwtTemplatesApi

All URIs are relative to *https://api.clerk.dev/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_jwt_template**](JwtTemplatesApi.md#create_jwt_template) | **POST** /jwt_templates | Create a JWT template
[**delete_jwt_template**](JwtTemplatesApi.md#delete_jwt_template) | **DELETE** /jwt_templates/{template_id} | Delete a Template
[**get_jwt_template**](JwtTemplatesApi.md#get_jwt_template) | **GET** /jwt_templates/{template_id} | Retrieve a template
[**list_jwt_templates**](JwtTemplatesApi.md#list_jwt_templates) | **GET** /jwt_templates | List all templates
[**update_jwt_template**](JwtTemplatesApi.md#update_jwt_template) | **PATCH** /jwt_templates/{template_id} | Update a JWT template



## create_jwt_template

> crate::models::JwtTemplate create_jwt_template(create_jwt_template_request)
Create a JWT template

Create a new JWT template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_jwt_template_request** | Option<[**CreateJwtTemplateRequest**](CreateJwtTemplateRequest.md)> |  |  |

### Return type

[**crate::models::JwtTemplate**](JWTTemplate.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_jwt_template

> crate::models::DeletedObject delete_jwt_template(template_id)
Delete a Template



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | JWT Template ID | [required] |

### Return type

[**crate::models::DeletedObject**](DeletedObject.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_jwt_template

> crate::models::JwtTemplate get_jwt_template(template_id)
Retrieve a template

Retrieve the details of a given JWT template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | JWT Template ID | [required] |

### Return type

[**crate::models::JwtTemplate**](JWTTemplate.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_jwt_templates

> Vec<crate::models::JwtTemplate> list_jwt_templates()
List all templates

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::JwtTemplate>**](JWTTemplate.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_jwt_template

> crate::models::JwtTemplate update_jwt_template(template_id, create_jwt_template_request)
Update a JWT template

Updates an existing JWT template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | The ID of the JWT template to update | [required] |
**create_jwt_template_request** | Option<[**CreateJwtTemplateRequest**](CreateJwtTemplateRequest.md)> |  |  |

### Return type

[**crate::models::JwtTemplate**](JWTTemplate.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

