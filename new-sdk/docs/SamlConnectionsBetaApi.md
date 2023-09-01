# \SamlConnectionsBetaApi

All URIs are relative to *https://api.clerk.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_saml_connection**](SamlConnectionsBetaApi.md#create_saml_connection) | **POST** /saml_connections | Create a SAML Connection
[**delete_saml_connection**](SamlConnectionsBetaApi.md#delete_saml_connection) | **DELETE** /saml_connections/{saml_connection_id} | Delete a SAML Connection
[**get_saml_connection**](SamlConnectionsBetaApi.md#get_saml_connection) | **GET** /saml_connections/{saml_connection_id} | Retrieve a SAML Connection by ID
[**list_saml_connections**](SamlConnectionsBetaApi.md#list_saml_connections) | **GET** /saml_connections | Get a list of SAML Connections for an instance
[**update_saml_connection**](SamlConnectionsBetaApi.md#update_saml_connection) | **PATCH** /saml_connections/{saml_connection_id} | Update a SAML Connection



## create_saml_connection

> crate::models::SamlConnection create_saml_connection(create_saml_connection_request)
Create a SAML Connection

Creates a new SAML Connection. <br/><br/> Note: This is a <b>Private Beta</b> feature and it is currently <b>hidden behind a feature flag</b>. Reach out to us via Intercom to try it out.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_saml_connection_request** | Option<[**CreateSamlConnectionRequest**](CreateSamlConnectionRequest.md)> |  |  |

### Return type

[**crate::models::SamlConnection**](SAMLConnection.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_saml_connection

> crate::models::DeletedObject delete_saml_connection(saml_connection_id)
Delete a SAML Connection

Deletes the given SAML Connection. <br/><br/> Note: This is a <b>Private Beta</b> feature and it is currently <b>hidden behind a feature flag</b>. Reach out to us via Intercom to try it out.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**saml_connection_id** | **String** | The ID of the SAML Connection to delete | [required] |

### Return type

[**crate::models::DeletedObject**](DeletedObject.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_saml_connection

> crate::models::SamlConnection get_saml_connection(saml_connection_id)
Retrieve a SAML Connection by ID

Fetches the SAML Connection whose ID matches the provided `id` in the path. <br/><br/> Note: This is a <b>Private Beta</b> feature and it is currently <b>hidden behind a feature flag</b>. Reach out to us via Intercom to try it out.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**saml_connection_id** | **String** | The ID of the SAML Connection | [required] |

### Return type

[**crate::models::SamlConnection**](SAMLConnection.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_saml_connections

> crate::models::SamlConnections list_saml_connections(limit, offset)
Get a list of SAML Connections for an instance

This request returns the list of SAML Connections for an instance. Results can be paginated using the optional `limit` and `offset` query parameters. The SAML Connections are ordered by descending creation date and the most recent will be returned first. <br/><br/> Note: This is a <b>Private Beta</b> feature and it is currently <b>hidden behind a feature flag</b>. Reach out to us via Intercom to try it out.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**f32**> | Applies a limit to the number of results returned. Can be used for paginating the results together with `offset`. Must be an integer greater than zero and less than 500. By default, if not supplied, a limit of 10 is used. |  |[default to 10]
**offset** | Option<**f32**> | Skip the first `offset` results when paginating. Needs to be an integer greater or equal to zero. To be used in conjunction with `limit`. |  |[default to 0]

### Return type

[**crate::models::SamlConnections**](SAMLConnections.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_saml_connection

> crate::models::SamlConnection update_saml_connection(saml_connection_id, update_saml_connection_request)
Update a SAML Connection

Updates an existing SAML Connection <br/><br/> Note: This is a <b>Private Beta</b> feature and it is currently <b>hidden behind a feature flag</b>. Reach out to us via Intercom to try it out.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**saml_connection_id** | **String** | The ID of the SAML Connection to update | [required] |
**update_saml_connection_request** | [**UpdateSamlConnectionRequest**](UpdateSamlConnectionRequest.md) |  | [required] |

### Return type

[**crate::models::SamlConnection**](SAMLConnection.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

