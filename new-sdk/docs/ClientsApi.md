# \ClientsApi

All URIs are relative to *https://api.clerk.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_client**](ClientsApi.md#get_client) | **GET** /clients/{client_id} | Get a client
[**get_client_last_active_session**](ClientsApi.md#get_client_last_active_session) | **GET** /clients/{client_id}/last_active_session | Get the last active session of a client
[**get_client_list**](ClientsApi.md#get_client_list) | **GET** /clients | List all clients
[**verify_client**](ClientsApi.md#verify_client) | **POST** /clients/verify | Verify a client



## get_client

> crate::models::Client get_client(client_id)
Get a client

Returns the details of a client.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | Client ID. | [required] |

### Return type

[**crate::models::Client**](Client.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_client_last_active_session

> crate::models::Session get_client_last_active_session(client_id)
Get the last active session of a client

Returns the details of the last active session of a client.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | Client ID. | [required] |

### Return type

[**crate::models::Session**](Session.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_client_list

> Vec<crate::models::Client> get_client_list(limit, offset)
List all clients

Returns a list of all clients. The clients are returned sorted by creation date, with the newest clients appearing first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**f32**> | Applies a limit to the number of results returned. Can be used for paginating the results together with `offset`. Must be an integer greater than zero and less than 500. By default, if not supplied, a limit of 10 is used. |  |[default to 10]
**offset** | Option<**f32**> | Skip the first `offset` results when paginating. Needs to be an integer greater or equal to zero. To be used in conjunction with `limit`. |  |[default to 0]

### Return type

[**Vec<crate::models::Client>**](Client.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_client

> crate::models::Client verify_client(verify_client_request)
Verify a client

Verifies the client in the provided token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verify_client_request** | Option<[**VerifyClientRequest**](VerifyClientRequest.md)> | Parameters. |  |

### Return type

[**crate::models::Client**](Client.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

