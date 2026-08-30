# \SamlConnectionsApi

All URIs are relative to *https://api.clerk.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_saml_connection**](SamlConnectionsApi.md#create_saml_connection) | **POST** /saml_connections | Create a SAML Connection
[**delete_saml_connection**](SamlConnectionsApi.md#delete_saml_connection) | **DELETE** /saml_connections/{saml_connection_id} | Delete a SAML Connection
[**get_saml_connection**](SamlConnectionsApi.md#get_saml_connection) | **GET** /saml_connections/{saml_connection_id} | Retrieve a SAML Connection by ID
[**list_saml_connections**](SamlConnectionsApi.md#list_saml_connections) | **GET** /saml_connections | Get a list of SAML Connections for an instance
[**update_saml_connection**](SamlConnectionsApi.md#update_saml_connection) | **PATCH** /saml_connections/{saml_connection_id} | Update a SAML Connection



## create_saml_connection

> models::SamlConnection create_saml_connection(create_saml_connection_request)
Create a SAML Connection

Create a new SAML Connection. Deprecated: Use the Enterprise Connections API instead. This endpoint will be removed in future versions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_saml_connection_request** | Option<[**CreateSamlConnectionRequest**](CreateSamlConnectionRequest.md)> |  |  |

### Return type

[**models::SamlConnection**](SAMLConnection.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_saml_connection

> models::DeletedObject delete_saml_connection(saml_connection_id)
Delete a SAML Connection

Deletes the SAML Connection whose ID matches the provided `id` in the path. Deprecated: Use the Enterprise Connections API instead. This endpoint will be removed in future versions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**saml_connection_id** | **String** | The ID of the SAML Connection to delete | [required] |

### Return type

[**models::DeletedObject**](DeletedObject.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_saml_connection

> models::SamlConnection get_saml_connection(saml_connection_id)
Retrieve a SAML Connection by ID

Fetches the SAML Connection whose ID matches the provided `saml_connection_id` in the path. Deprecated: Use the Enterprise Connections API instead. This endpoint will be removed in future versions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**saml_connection_id** | **String** | The ID of the SAML Connection | [required] |

### Return type

[**models::SamlConnection**](SAMLConnection.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_saml_connections

> models::SamlConnections list_saml_connections(limit, offset, query, order_by, organization_id)
Get a list of SAML Connections for an instance

Returns the list of SAML Connections for an instance. Results can be paginated using the optional `limit` and `offset` query parameters. The SAML Connections are ordered by descending creation date and the most recent will be returned first. Deprecated: Use the Enterprise Connections API instead. This endpoint will be removed in future versions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**u32**> | Applies a limit to the number of results returned. Can be used for paginating the results together with `offset`. |  |[default to 10]
**offset** | Option<**u32**> | Skip the first `offset` results when paginating. Needs to be an integer greater or equal to zero. To be used in conjunction with `limit`. |  |[default to 0]
**query** | Option<**String**> | Returns SAML connections that have a name that matches the given query, via case-insensitive partial match. |  |
**order_by** | Option<**String**> | Sorts organizations memberships by phone_number, email_address, created_at, first_name, last_name or username. By prepending one of those values with + or -, we can choose to sort in ascending (ASC) or descending (DESC) order. |  |
**organization_id** | Option<[**Vec<String>**](String.md)> | Returns SAML connections that have an associated organization ID to the given organizations. For each organization ID, the `+` and `-` can be prepended to the ID, which denote whether the respective organization should be included or excluded from the result set. Accepts up to 100 organization IDs. |  |

### Return type

[**models::SamlConnections**](SAMLConnections.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_saml_connection

> models::SamlConnection update_saml_connection(saml_connection_id, update_saml_connection_request)
Update a SAML Connection

Updates the SAML Connection whose ID matches the provided `id` in the path. Deprecated: Use the Enterprise Connections API instead. This endpoint will be removed in future versions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**saml_connection_id** | **String** | The ID of the SAML Connection to update | [required] |
**update_saml_connection_request** | [**UpdateSamlConnectionRequest**](UpdateSamlConnectionRequest.md) |  | [required] |

### Return type

[**models::SamlConnection**](SAMLConnection.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

