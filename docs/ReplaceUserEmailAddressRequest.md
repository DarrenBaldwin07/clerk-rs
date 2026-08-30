# ReplaceUserEmailAddressRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email_address** | **String** | The new email address. Must adhere to the RFC 5322 specification for email address format. |
**identification_status** | Option<**IdentificationStatus**> | Controls the status of the replacement email address. Defaults to `verified`. Set to `reserved` to create it reserved (unverified but usable for sign-in and locked so no other user can claim it), or to `unverified` to create it neither usable for sign-in nor locked.  **Warning:** `unverified` can lock the user out of their account. An unverified email address cannot be used to sign in, so if the user has no other verified or reserved identifier, they will be unable to authenticate and unable to verify this address. Prefer `reserved` unless you specifically need the address left unclaimed — for example so that another user can also hold it until one of them verifies it. (enum: verified, reserved, unverified) | [optional][default to Verified]
**notify_primary_email_address_changed** | Option<**bool**> | If set to `true`, the user's previous primary email address is notified that the primary email address has changed. No notification is sent when the replacement is the user's current primary email address. By default, no notification is sent. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


