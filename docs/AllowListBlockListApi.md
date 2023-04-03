# \AllowListBlockListApi

All URIs are relative to *https://api.clerk.dev/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_allowlist_identifier**](AllowListBlockListApi.md#create_allowlist_identifier) | **POST** /allowlist_identifiers | Add identifier to the allow-list
[**create_blocklist_identifier**](AllowListBlockListApi.md#create_blocklist_identifier) | **POST** /blocklist_identifiers | Add identifier to the block-list
[**delete_allowlist_identifier**](AllowListBlockListApi.md#delete_allowlist_identifier) | **DELETE** /allowlist_identifiers/{identifier_id} | Delete identifier from allow-list
[**delete_blocklist_identifier**](AllowListBlockListApi.md#delete_blocklist_identifier) | **DELETE** /blocklist_identifiers/{identifier_id} | Delete identifier from block-list
[**list_allowlist_identifiers**](AllowListBlockListApi.md#list_allowlist_identifiers) | **GET** /allowlist_identifiers | List all identifiers on the allow-list
[**list_blocklist_identifiers**](AllowListBlockListApi.md#list_blocklist_identifiers) | **GET** /blocklist_identifiers | List all identifiers on the block-list



## create_allowlist_identifier

> crate::models::AllowlistIdentifier create_allowlist_identifier()
Add identifier to the allow-list

Create an identifier allowed to sign up to an instance

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AllowlistIdentifier**](AllowlistIdentifier.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_blocklist_identifier

> crate::models::BlocklistIdentifier create_blocklist_identifier()
Add identifier to the block-list

Create an identifier that is blocked from accessing an instance

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::BlocklistIdentifier**](BlocklistIdentifier.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_allowlist_identifier

> crate::models::DeletedObject delete_allowlist_identifier(identifier_id)
Delete identifier from allow-list

Delete an identifier from the instance allow-list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier_id** | **String** | The ID of the identifier to delete from the allow-list | [required] |

### Return type

[**crate::models::DeletedObject**](DeletedObject.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_blocklist_identifier

> crate::models::DeletedObject delete_blocklist_identifier(identifier_id)
Delete identifier from block-list

Delete an identifier from the instance block-list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier_id** | **String** | The ID of the identifier to delete from the block-list | [required] |

### Return type

[**crate::models::DeletedObject**](DeletedObject.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_allowlist_identifiers

> Vec<crate::models::AllowlistIdentifier> list_allowlist_identifiers()
List all identifiers on the allow-list

Get a list of all identifiers allowed to sign up to an instance

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::AllowlistIdentifier>**](AllowlistIdentifier.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_blocklist_identifiers

> crate::models::BlocklistIdentifiers list_blocklist_identifiers()
List all identifiers on the block-list

Get a list of all identifiers which are not allowed to access an instance

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::BlocklistIdentifiers**](BlocklistIdentifiers.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

