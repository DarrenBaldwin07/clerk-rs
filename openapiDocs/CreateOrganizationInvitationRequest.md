# CreateOrganizationInvitationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email_address** | **String** | The email address of the new member that is going to be invited to the organization | 
**inviter_user_id** | **String** | The ID of the user that invites the new member to the organization. Must be an administrator in the organization. | 
**role** | **String** | The role of the new member in the organization | 
**public_metadata** | Option<[**serde_json::Value**](.md)> | Metadata saved on the organization invitation, read-only from the Frontend API and fully accessible (read/write) from the Backend API. | [optional]
**redirect_url** | Option<**String**> | Optional URL that the invitee will be redirected to once they accept the invitation by clicking the join link in the invitation email. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


