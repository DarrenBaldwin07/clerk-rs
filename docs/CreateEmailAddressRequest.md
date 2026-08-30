# CreateEmailAddressRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_id** | **String** | The ID representing the user |
**email_address** | **String** | The new email address. Must adhere to the RFC 5322 specification for email address format. |
**verified** | Option<**bool**> | When created, the email address will be marked as verified. | [optional]
**primary** | Option<**bool**> | Create this email address as the primary email address for the user. Default: false, unless it is the first email address. | [optional]
**notify_primary_email_address_changed** | Option<**bool**> | If set to `true` and the email address is created as the user's new primary, the previous primary email address is notified of the change. By default, no notification is sent. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


