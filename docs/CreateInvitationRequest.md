# CreateInvitationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email_address** | **String** | The email address the invitation will be sent to |
**public_metadata** | Option<**std::collections::HashMap<String, serde_json::Value>**> | Metadata that will be attached to the newly created invitation. The value of this property should be a well-formed JSON object. Once the user accepts the invitation and signs up, these metadata will end up in the user's public metadata. | [optional]
**redirect_url** | Option<**String**> | Optional URL which specifies where to redirect the user once they click the invitation link. This is only required if you have implemented a [custom flow](https://clerk.com/docs/authentication/invitations#custom-flow) and you're not using Clerk Hosted Pages or Clerk Components. | [optional]
**notify** | Option<**bool**> | Optional flag which denotes whether an email invitation should be sent to the given email address. Defaults to `true`. | [optional][default to true]
**ignore_existing** | Option<**bool**> | Whether an invitation should be created if there is already an existing invitation for this email address, or it's claimed by another user. | [optional][default to false]
**expires_in_days** | Option<**u32**> | The number of days the invitation will be valid for. By default, the invitation expires after 30 days. | [optional]
**template_slug** | Option<**TemplateSlug**> | The slug of the email template to use for the invitation email. (enum: invitation, waitlist_invitation) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


