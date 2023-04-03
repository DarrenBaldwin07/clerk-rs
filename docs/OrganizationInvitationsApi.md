# \OrganizationInvitationsApi

All URIs are relative to *https://api.clerk.dev/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_organization_invitation**](OrganizationInvitationsApi.md#create_organization_invitation) | **POST** /organizations/{organization_id}/invitations | Create and send an organization invitation
[**list_pending_organization_invitations**](OrganizationInvitationsApi.md#list_pending_organization_invitations) | **GET** /organizations/{organization_id}/invitations/pending | Get a list of pending organization invitations
[**revoke_organization_invitation**](OrganizationInvitationsApi.md#revoke_organization_invitation) | **POST** /organizations/{organization_id}/invitations/{invitation_id}/revoke | Revoke a pending organization invitation



## create_organization_invitation

> crate::models::OrganizationInvitation create_organization_invitation(organization_id, create_organization_invitation_request)
Create and send an organization invitation

Creates a new organization invitation and sends an email to the provided `email_address` with a link to accept the invitation and join the organization. You can specify the `role` for the invited organization member.  New organization invitations get a \"pending\" status until they are revoked by an organization administrator or accepted by the invitee.  The request body supports passing an optional `redirect_url` parameter. When the invited user clicks the link to accept the invitation, they will be redirected to the URL provided. Use this parameter to implement a custom invitation acceptance flow.  You must specify the ID of the user that will send the invitation with the `inviter_user_id` parameter. That user must be a member with administrator privileges in the organization. Only \"admin\" members can create organization invitations.  You can optionally provide public metadata for the organization invitation. These metadata are visible by both the Frontend and the Backend. When the organization invitation is accepted, the metadata will be transferred to the newly created organization membership.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** | The ID of the organization for which to send the invitation | [required] |
**create_organization_invitation_request** | [**CreateOrganizationInvitationRequest**](CreateOrganizationInvitationRequest.md) |  | [required] |

### Return type

[**crate::models::OrganizationInvitation**](OrganizationInvitation.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_pending_organization_invitations

> crate::models::OrganizationInvitations list_pending_organization_invitations(organization_id, limit, offset)
Get a list of pending organization invitations

This request returns the list of organization invitations with \"pending\" status. These are the organization invitations that can still be used to join the organization, but have not been accepted by the invited user yet. Results can be paginated using the optional `limit` and `offset` query parameters. The organization invitations are ordered by descending creation date. Most recent invitations will be returned first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** | The organization ID. | [required] |
**limit** | Option<**f32**> | Applies a limit to the number of results returned. Can be used for paginating the results together with `offset`. Must be an integer greater than zero and less than 500. By default, if not supplied, a limit of 10 is used. |  |[default to 10]
**offset** | Option<**f32**> | Skip the first `offset` results when paginating. Needs to be an integer greater or equal to zero. To be used in conjunction with `limit`. |  |[default to 0]

### Return type

[**crate::models::OrganizationInvitations**](OrganizationInvitations.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_organization_invitation

> crate::models::OrganizationInvitation revoke_organization_invitation(organization_id, invitation_id, revoke_organization_invitation_request)
Revoke a pending organization invitation

Use this request to revoke a previously issued organization invitation. Revoking an organization invitation makes it invalid; the invited user will no longer be able to join the organization with the revoked invitation. Only organization invitations with \"pending\" status can be revoked. The request needs the `requesting_user_id` parameter to specify the user which revokes the invitation. Only users with \"admin\" role can revoke invitations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** | The organization ID. | [required] |
**invitation_id** | **String** | The organization invitation ID. | [required] |
**revoke_organization_invitation_request** | [**RevokeOrganizationInvitationRequest**](RevokeOrganizationInvitationRequest.md) |  | [required] |

### Return type

[**crate::models::OrganizationInvitation**](OrganizationInvitation.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

