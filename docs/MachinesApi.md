# \MachinesApi

All URIs are relative to *https://api.clerk.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_machine**](MachinesApi.md#create_machine) | **POST** /machines | Create a machine
[**create_machine_scope**](MachinesApi.md#create_machine_scope) | **POST** /machines/{machine_id}/scopes | Create a machine scope
[**delete_machine**](MachinesApi.md#delete_machine) | **DELETE** /machines/{machine_id} | Delete a machine
[**delete_machine_scope**](MachinesApi.md#delete_machine_scope) | **DELETE** /machines/{machine_id}/scopes/{other_machine_id} | Delete a machine scope
[**get_machine**](MachinesApi.md#get_machine) | **GET** /machines/{machine_id} | Retrieve a machine
[**get_machine_secret_key**](MachinesApi.md#get_machine_secret_key) | **GET** /machines/{machine_id}/secret_key | Retrieve a machine secret key
[**list_machines**](MachinesApi.md#list_machines) | **GET** /machines | Get a list of machines for an instance
[**rotate_machine_secret_key**](MachinesApi.md#rotate_machine_secret_key) | **POST** /machines/{machine_id}/secret_key/rotate | Rotate a machine's secret key
[**update_machine**](MachinesApi.md#update_machine) | **PATCH** /machines/{machine_id} | Update a machine



## create_machine

> models::CreateMachine200Response create_machine(create_machine_request)
Create a machine

Creates a new machine.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_machine_request** | Option<[**CreateMachineRequest**](CreateMachineRequest.md)> |  |  |

### Return type

[**models::CreateMachine200Response**](CreateMachine_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_machine_scope

> models::MachineScope create_machine_scope(machine_id, create_machine_scope_request)
Create a machine scope

Creates a new machine scope, allowing the specified machine to access another machine. Maximum of 150 scopes per machine.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**machine_id** | **String** | The ID of the machine that will have access to another machine | [required] |
**create_machine_scope_request** | Option<[**CreateMachineScopeRequest**](CreateMachineScopeRequest.md)> |  |  |

### Return type

[**models::MachineScope**](MachineScope.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_machine

> models::DeleteMachine200Response delete_machine(machine_id)
Delete a machine

Deletes a machine.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**machine_id** | **String** | The ID of the machine to delete | [required] |

### Return type

[**models::DeleteMachine200Response**](DeleteMachine_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_machine_scope

> models::DeleteMachineScope200Response delete_machine_scope(machine_id, other_machine_id)
Delete a machine scope

Deletes a machine scope, removing access from one machine to another.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**machine_id** | **String** | The ID of the machine that has access to another machine | [required] |
**other_machine_id** | **String** | The ID of the machine that is being accessed | [required] |

### Return type

[**models::DeleteMachineScope200Response**](DeleteMachineScope_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_machine

> models::Machine get_machine(machine_id)
Retrieve a machine

Returns the details of a machine.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**machine_id** | **String** | The ID of the machine to retrieve | [required] |

### Return type

[**models::Machine**](Machine.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_machine_secret_key

> models::GetMachineSecretKey200Response get_machine_secret_key(machine_id)
Retrieve a machine secret key

Returns the secret key for a machine.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**machine_id** | **String** | The ID of the machine to retrieve the secret key for | [required] |

### Return type

[**models::GetMachineSecretKey200Response**](GetMachineSecretKey_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_machines

> models::ListMachines200Response list_machines(limit, offset, query, order_by)
Get a list of machines for an instance

This request returns the list of machines for an instance. The machines are ordered by descending creation date (i.e. most recent machines will be returned first)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**u32**> | Applies a limit to the number of results returned. Can be used for paginating the results together with `offset`. |  |[default to 10]
**offset** | Option<**u32**> | Skip the first `offset` results when paginating. Needs to be an integer greater or equal to zero. To be used in conjunction with `limit`. |  |[default to 0]
**query** | Option<**String**> | Returns machines with ID or name that match the given query. Uses exact match for machine ID and partial match for name. |  |
**order_by** | Option<**String**> | Allows to return machines in a particular order. You can order the returned machines by their `name` or `created_at`. To specify the direction, use the `+` or `-` symbols prepended to the property to order by. For example, to return machines in descending order by `created_at`, use `-created_at`. If you don't use `+` or `-`, then `+` is implied. Defaults to `-created_at`. |  |[default to -created_at]

### Return type

[**models::ListMachines200Response**](ListMachines_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rotate_machine_secret_key

> models::GetMachineSecretKey200Response rotate_machine_secret_key(machine_id, rotate_machine_secret_key_request)
Rotate a machine's secret key

Rotates the machine's secret key. When the secret key is rotated, make sure to update it in your machine/application. The previous secret key will remain valid for the duration specified by the previous_token_ttl parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**machine_id** | **String** | The ID of the machine to rotate the secret key for | [required] |
**rotate_machine_secret_key_request** | [**RotateMachineSecretKeyRequest**](RotateMachineSecretKeyRequest.md) |  | [required] |

### Return type

[**models::GetMachineSecretKey200Response**](GetMachineSecretKey_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_machine

> models::Machine update_machine(machine_id, update_machine_request)
Update a machine

Updates an existing machine. Only the provided fields will be updated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**machine_id** | **String** | The ID of the machine to update | [required] |
**update_machine_request** | Option<[**UpdateMachineRequest**](UpdateMachineRequest.md)> |  |  |

### Return type

[**models::Machine**](Machine.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

