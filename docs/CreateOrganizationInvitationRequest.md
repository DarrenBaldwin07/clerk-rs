# CreateOrganizationInvitationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email_address** | **String** | The email address of the new member that is going to be invited to the organization |
**inviter_user_id** | Option<**String**> | The ID of the user that invites the new member to the organization. Must be an administrator in the organization. | [optional]
**role** | **String** | The role of the new member in the organization |
**public_metadata** | Option<**std::collections::HashMap<String, serde_json::Value>**> | Metadata saved on the organization invitation, read-only from the Frontend API and fully accessible (read/write) from the Backend API. When the organization invitation is accepted, the metadata will be transferred to the newly created organization membership. | [optional]
**private_metadata** | Option<**std::collections::HashMap<String, serde_json::Value>**> | Metadata saved on the organization invitation, fully accessible (read/write) from the Backend API but not visible from the Frontend API. When the organization invitation is accepted, the metadata will be transferred to the newly created organization membership. | [optional]
**redirect_url** | Option<**String**> | Optional URL that the invitee will be redirected to once they accept the invitation by clicking the join link in the invitation email. | [optional]
**expires_in_days** | Option<**u32**> | The number of days the invitation will be valid for. By default, the invitation has a 30 days expire. | [optional]
**notify** | Option<**bool**> | Optional flag which denotes whether an email invitation should be sent to the given email address. Defaults to `true`. | [optional][default to true]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


