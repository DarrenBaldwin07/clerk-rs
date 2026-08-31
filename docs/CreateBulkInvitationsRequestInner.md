# CreateBulkInvitationsRequestInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email_address** | **String** | The email address the invitation will be sent to |
**public_metadata** | Option<**std::collections::HashMap<String, serde_json::Value>**> | Metadata that will be attached to the newly created invitation. The value of this property should be a well-formed JSON object. Once the user accepts the invitation and signs up, these metadata will end up in the user's public metadata. | [optional]
**redirect_url** | Option<**String**> | The URL where the user is redirected upon visiting the invitation link, where they can accept the invitation. Required if you have implemented a [custom flow for handling application invitations](/docs/custom-flows/invitations). | [optional]
**notify** | Option<**bool**> | Optional flag which denotes whether an email invitation should be sent to the given email address. Defaults to true. | [optional][default to true]
**ignore_existing** | Option<**bool**> | Whether an invitation should be created if there is already an existing invitation for this email address, or it's claimed by another user. | [optional][default to false]
**expires_in_days** | Option<**u32**> | The number of days the invitation will be valid for. By default, the invitation expires after 30 days. | [optional]
**template_slug** | Option<**TemplateSlug**> | The slug of the email template to use for the invitation email. (enum: invitation, waitlist_invitation) | [optional][default to Invitation]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


