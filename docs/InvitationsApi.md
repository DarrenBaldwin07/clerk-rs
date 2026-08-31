# \InvitationsApi

All URIs are relative to *https://api.clerk.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_bulk_invitations**](InvitationsApi.md#create_bulk_invitations) | **POST** /invitations/bulk | Create multiple invitations
[**create_invitation**](InvitationsApi.md#create_invitation) | **POST** /invitations | Create an invitation
[**list_invitations**](InvitationsApi.md#list_invitations) | **GET** /invitations | List all invitations
[**revoke_invitation**](InvitationsApi.md#revoke_invitation) | **POST** /invitations/{invitation_id}/revoke | Revokes an invitation



## create_bulk_invitations

> Vec<models::Invitation> create_bulk_invitations(create_bulk_invitations_request_inner)
Create multiple invitations

Use this API operation to create multiple invitations for the provided email addresses. You can choose to send the invitations as emails by setting the `notify` parameter to `true`. There cannot be an existing invitation for any of the email addresses you provide unless you set `ignore_existing` to `true` for specific email addresses. Please note that there must be no existing user for any of the email addresses you provide, and this rule cannot be bypassed.  This endpoint is limited to a maximum of 10 invitations per API call. If you need to send more invitations, please make multiple requests.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_bulk_invitations_request_inner** | Option<[**Vec<models::CreateBulkInvitationsRequestInner>**](CreateBulkInvitationsRequestInner.md)> | Required parameters |  |

### Return type

[**Vec<models::Invitation>**](Invitation.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_invitation

> models::Invitation create_invitation(create_invitation_request)
Create an invitation

Creates a new invitation for the given email address and sends the invitation email. Keep in mind that you cannot create an invitation if there is already one for the given email address. Also, trying to create an invitation for an email address that already exists in your application will result to an error.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_invitation_request** | Option<[**CreateInvitationRequest**](CreateInvitationRequest.md)> | Required parameters |  |

### Return type

[**models::Invitation**](Invitation.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_invitations

> Vec<models::Invitation> list_invitations(status, query, order_by, paginated, limit, offset)
List all invitations

Returns all non-revoked invitations for your application, sorted by creation date

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<**String**> | Filter invitations based on their status |  |
**query** | Option<**String**> | Filter invitations based on their `email_address` or `id` |  |
**order_by** | Option<**String**> | Allows to return invitations in a particular order. At the moment, you can order the returned invitations either by their `created_at`, `email_address` or `expires_at`. In order to specify the direction, you can use the `+/-` symbols prepended in the property to order by. For example, if you want invitations to be returned in descending order according to their `created_at` property, you can use `-created_at`. If you don't use `+` or `-`, then `+` is implied. Defaults to `-created_at`. |  |[default to -created_at]
**paginated** | Option<**bool**> | Whether to paginate the results. If true, the results will be paginated. If false, the results will not be paginated. |  |
**limit** | Option<**u32**> | Applies a limit to the number of results returned. Can be used for paginating the results together with `offset`. |  |[default to 10]
**offset** | Option<**u32**> | Skip the first `offset` results when paginating. Needs to be an integer greater or equal to zero. To be used in conjunction with `limit`. |  |[default to 0]

### Return type

[**Vec<models::Invitation>**](Invitation.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_invitation

> models::RevokeInvitation200Response revoke_invitation(invitation_id)
Revokes an invitation

Revokes the given invitation. Revoking an invitation will prevent the user from using the invitation link that was sent to them. However, it doesn't prevent the user from signing up if they follow the sign up flow. Only active (i.e. non-revoked) invitations can be revoked.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invitation_id** | **String** | The ID of the invitation to be revoked | [required] |

### Return type

[**models::RevokeInvitation200Response**](RevokeInvitation_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

