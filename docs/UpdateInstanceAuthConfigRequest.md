# UpdateInstanceAuthConfigRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**restricted_to_allowlist** | Option<**bool**> | Whether sign up is restricted to email addresses, phone numbers and usernames that are on the allowlist. | [optional][default to false]
**from_email_address** | Option<**String**> | The local part of the email address from which authentication-related emails (e.g. OTP code, magic links) will be sent. Only alphanumeric values are allowed. Note that this value should contain only the local part of the address (e.g. `foo` for `foo@example.com`). | [optional]
**progressive_sign_up** | Option<**bool**> | Enable the Progressive Sign Up algorithm. This feature is deprecated, please contact support if you need assistance. | [optional]
**test_mode** | Option<**bool**> | Toggles test mode for this instance, allowing the use of test email addresses and phone numbers. Defaults to true for development instances. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


