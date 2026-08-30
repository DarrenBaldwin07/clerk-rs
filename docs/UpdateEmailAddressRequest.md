# UpdateEmailAddressRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**verified** | Option<**bool**> | The email address will be marked as verified. | [optional]
**primary** | Option<**bool**> | Set this email address as the primary email address for the user. | [optional]
**notify_primary_email_address_changed** | Option<**bool**> | If set to `true` and this update makes the email address the user's new primary, the previous primary email address is notified of the change. By default, no notification is sent. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


