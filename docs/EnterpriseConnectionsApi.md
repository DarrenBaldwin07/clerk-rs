# \EnterpriseConnectionsApi

All URIs are relative to *https://api.clerk.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_enterprise_connection**](EnterpriseConnectionsApi.md#create_enterprise_connection) | **POST** /enterprise_connections | Create an enterprise connection
[**create_enterprise_connection_test_run**](EnterpriseConnectionsApi.md#create_enterprise_connection_test_run) | **POST** /enterprise_connections/{enterprise_connection_id}/test_runs | Create an enterprise connection test run
[**delete_enterprise_connection**](EnterpriseConnectionsApi.md#delete_enterprise_connection) | **DELETE** /enterprise_connections/{enterprise_connection_id} | Delete an enterprise connection
[**get_enterprise_connection**](EnterpriseConnectionsApi.md#get_enterprise_connection) | **GET** /enterprise_connections/{enterprise_connection_id} | Retrieve an enterprise connection
[**list_enterprise_connection_test_runs**](EnterpriseConnectionsApi.md#list_enterprise_connection_test_runs) | **GET** /enterprise_connections/{enterprise_connection_id}/test_runs | List enterprise connection test runs
[**list_enterprise_connections**](EnterpriseConnectionsApi.md#list_enterprise_connections) | **GET** /enterprise_connections | List enterprise connections
[**update_enterprise_connection**](EnterpriseConnectionsApi.md#update_enterprise_connection) | **PATCH** /enterprise_connections/{enterprise_connection_id} | Update an enterprise connection



## create_enterprise_connection

> models::EnterpriseConnection create_enterprise_connection(create_enterprise_connection_request)
Create an enterprise connection

Create a new enterprise connection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_enterprise_connection_request** | Option<[**CreateEnterpriseConnectionRequest**](CreateEnterpriseConnectionRequest.md)> |  |  |

### Return type

[**models::EnterpriseConnection**](EnterpriseConnection.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_enterprise_connection_test_run

> models::EnterpriseConnectionTestRunResponse create_enterprise_connection_test_run(enterprise_connection_id)
Create an enterprise connection test run

Returns a short-lived URL that starts the IdP test flow for this connection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**enterprise_connection_id** | **String** | The ID of the enterprise connection | [required] |

### Return type

[**models::EnterpriseConnectionTestRunResponse**](EnterpriseConnectionTestRunResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_enterprise_connection

> models::DeletedObject delete_enterprise_connection(enterprise_connection_id)
Delete an enterprise connection

Deletes the enterprise connection whose ID matches the provided `enterprise_connection_id` in the path.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**enterprise_connection_id** | **String** | The ID of the enterprise connection to delete | [required] |

### Return type

[**models::DeletedObject**](DeletedObject.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_enterprise_connection

> models::EnterpriseConnection get_enterprise_connection(enterprise_connection_id)
Retrieve an enterprise connection

Fetches the enterprise connection whose ID matches the provided `enterprise_connection_id` in the path.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**enterprise_connection_id** | **String** | The ID of the enterprise connection | [required] |

### Return type

[**models::EnterpriseConnection**](EnterpriseConnection.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_enterprise_connection_test_runs

> models::EnterpriseConnectionTestRuns list_enterprise_connection_test_runs(enterprise_connection_id, status, limit, offset)
List enterprise connection test runs

Returns a paginated list of SAML or OIDC debug test runs for an enterprise connection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**enterprise_connection_id** | **String** | The ID of the enterprise connection | [required] |
**status** | Option<[**Vec<String>**](String.md)> | Filter test runs by status (may be repeated) |  |
**limit** | Option<**u32**> | Applies a limit to the number of results returned. Can be used for paginating the results together with `offset`. |  |[default to 10]
**offset** | Option<**u32**> | Skip the first `offset` results when paginating. Needs to be an integer greater or equal to zero. To be used in conjunction with `limit`. |  |[default to 0]

### Return type

[**models::EnterpriseConnectionTestRuns**](EnterpriseConnectionTestRuns.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_enterprise_connections

> models::EnterpriseConnections list_enterprise_connections(limit, offset, organization_id, active)
List enterprise connections

Returns the list of enterprise connections for the instance. Results can be paginated using the optional `limit` and `offset` query parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**u32**> | Applies a limit to the number of results returned. Can be used for paginating the results together with `offset`. |  |[default to 10]
**offset** | Option<**u32**> | Skip the first `offset` results when paginating. Needs to be an integer greater or equal to zero. To be used in conjunction with `limit`. |  |[default to 0]
**organization_id** | Option<**String**> | Filter enterprise connections by organization ID |  |
**active** | Option<**bool**> | Filter by active status. If true, only active connections are returned. If false, only inactive connections are returned. If omitted, all connections are returned. |  |

### Return type

[**models::EnterpriseConnections**](EnterpriseConnections.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_enterprise_connection

> models::EnterpriseConnection update_enterprise_connection(enterprise_connection_id, update_enterprise_connection_request)
Update an enterprise connection

Updates the enterprise connection whose ID matches the provided `enterprise_connection_id` in the path. When enabling the connection (setting `active` to true), any existing verified organization domains that match the connection's domains (e.g. used for enrollment modes like automatic invitation) may be deleted so the connection can be enabled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**enterprise_connection_id** | **String** | The ID of the enterprise connection to update | [required] |
**update_enterprise_connection_request** | [**UpdateEnterpriseConnectionRequest**](UpdateEnterpriseConnectionRequest.md) |  | [required] |

### Return type

[**models::EnterpriseConnection**](EnterpriseConnection.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

