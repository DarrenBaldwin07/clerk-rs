# \WaitlistEntriesApi

All URIs are relative to *https://api.clerk.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_bulk_waitlist_entries**](WaitlistEntriesApi.md#create_bulk_waitlist_entries) | **POST** /waitlist_entries/bulk | Create multiple waitlist entries
[**create_waitlist_entry**](WaitlistEntriesApi.md#create_waitlist_entry) | **POST** /waitlist_entries | Create a waitlist entry
[**delete_waitlist_entry**](WaitlistEntriesApi.md#delete_waitlist_entry) | **DELETE** /waitlist_entries/{waitlist_entry_id} | Delete a pending waitlist entry
[**invite_waitlist_entry**](WaitlistEntriesApi.md#invite_waitlist_entry) | **POST** /waitlist_entries/{waitlist_entry_id}/invite | Invite a waitlist entry
[**list_waitlist_entries**](WaitlistEntriesApi.md#list_waitlist_entries) | **GET** /waitlist_entries | List all waitlist entries
[**reject_waitlist_entry**](WaitlistEntriesApi.md#reject_waitlist_entry) | **POST** /waitlist_entries/{waitlist_entry_id}/reject | Reject a waitlist entry



## create_bulk_waitlist_entries

> Vec<models::WaitlistEntry> create_bulk_waitlist_entries(create_bulk_waitlist_entries_request_inner)
Create multiple waitlist entries

Creates multiple waitlist entries for the provided email addresses. You can choose whether to send confirmation emails by setting the `notify` parameter to `true` or `false` for each entry. If the `notify` parameter is omitted, it defaults to `true`.  If an email address is already on the waitlist, no new entry will be created and the existing waitlist entry will be returned. Duplicate email addresses within the same request are not allowed.  This endpoint is limited to a maximum of 50 entries per API call. If you need to add more entries, please make multiple requests.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_bulk_waitlist_entries_request_inner** | Option<[**Vec<models::CreateBulkWaitlistEntriesRequestInner>**](CreateBulkWaitlistEntriesRequestInner.md)> | Required parameters |  |

### Return type

[**Vec<models::WaitlistEntry>**](WaitlistEntry.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_waitlist_entry

> models::WaitlistEntry create_waitlist_entry(create_waitlist_entry_request)
Create a waitlist entry

Creates a new waitlist entry for the given email address. If the email address is already on the waitlist, no new entry will be created and the existing waitlist entry will be returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_waitlist_entry_request** | Option<[**CreateWaitlistEntryRequest**](CreateWaitlistEntryRequest.md)> |  |  |

### Return type

[**models::WaitlistEntry**](WaitlistEntry.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_waitlist_entry

> models::DeletedObject delete_waitlist_entry(waitlist_entry_id)
Delete a pending waitlist entry

Delete a pending waitlist entry.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**waitlist_entry_id** | **String** | The ID of the waitlist entry to delete | [required] |

### Return type

[**models::DeletedObject**](DeletedObject.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invite_waitlist_entry

> models::WaitlistEntry invite_waitlist_entry(waitlist_entry_id, invite_waitlist_entry_request)
Invite a waitlist entry

Send an invite to the email address in a waitlist entry.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**waitlist_entry_id** | **String** | The ID of the waitlist entry to invite | [required] |
**invite_waitlist_entry_request** | Option<[**InviteWaitlistEntryRequest**](InviteWaitlistEntryRequest.md)> |  |  |

### Return type

[**models::WaitlistEntry**](WaitlistEntry.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_waitlist_entries

> models::ListWaitlistEntries200Response list_waitlist_entries(limit, offset, query, status, order_by)
List all waitlist entries

Retrieve a list of waitlist entries for the instance. Entries are ordered by creation date in descending order by default. Supports filtering by email address or status and pagination with limit and offset parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**u32**> | Applies a limit to the number of results returned. Can be used for paginating the results together with `offset`. |  |[default to 10]
**offset** | Option<**u32**> | Skip the first `offset` results when paginating. Needs to be an integer greater or equal to zero. To be used in conjunction with `limit`. |  |[default to 0]
**query** | Option<**String**> | Filter waitlist entries by `email_address` or `id` |  |
**status** | Option<**String**> | Filter waitlist entries by their status |  |
**order_by** | Option<**String**> | Specify the order of results. Supported values are: - `created_at` - `email_address` - `invited_at`  Use `+` for ascending or `-` for descending order. Defaults to `-created_at`. |  |[default to -created_at]

### Return type

[**models::ListWaitlistEntries200Response**](ListWaitlistEntries_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reject_waitlist_entry

> models::WaitlistEntry reject_waitlist_entry(waitlist_entry_id)
Reject a waitlist entry

Reject a waitlist entry.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**waitlist_entry_id** | **String** | The ID of the waitlist entry to reject | [required] |

### Return type

[**models::WaitlistEntry**](WaitlistEntry.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

