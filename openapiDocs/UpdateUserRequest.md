# UpdateUserRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**external_id** | Option<**String**> | The ID of the user as used in your external systems or your previous authentication solution. Must be unique across your instance. | [optional]
**first_name** | Option<**String**> | The first name to assign to the user | [optional]
**last_name** | Option<**String**> | The last name to assign to the user | [optional]
**primary_email_address_id** | Option<**String**> | The ID of the email address to set as primary. It must be verified, and present on the current user. | [optional]
**primary_phone_number_id** | Option<**String**> | The ID of the phone number to set as primary. It must be verified, and present on the current user. | [optional]
**primary_web3_wallet_id** | Option<**String**> | The ID of the web3 wallets to set as primary. It must be verified, and present on the current user. | [optional]
**username** | Option<**String**> | The username to give to the user. It must be unique across your instance. | [optional]
**profile_image_id** | Option<**String**> | The ID of the image to set as the user's profile image | [optional]
**password** | Option<**String**> | The plaintext password to give the user. Must be at least 8 characters long, and can not be in any list of hacked passwords. | [optional]
**totp_secret** | Option<**String**> | In case TOTP is configured on the instance, you can provide the secret to enable it on the specific user without the need to reset it. Please note that currently the supported options are: * Period: 30 seconds * Code length: 6 digits * Algorithm: SHA1 | [optional]
**backup_codes** | Option<**Vec<String>**> | If Backup Codes are configured on the instance, you can provide them to enable it on the specific user without the need to reset them. You must provide the backup codes in plain format or the corresponding bcrypt digest. | [optional]
**public_metadata** | Option<[**serde_json::Value**](.md)> | Metadata saved on the user, that is visible to both your Frontend and Backend APIs | [optional]
**private_metadata** | Option<[**serde_json::Value**](.md)> | Metadata saved on the user, that is only visible to your Backend API | [optional]
**unsafe_metadata** | Option<[**serde_json::Value**](.md)> | Metadata saved on the user, that can be updated from both the Frontend and Backend APIs. Note: Since this data can be modified from the frontend, it is not guaranteed to be safe. | [optional]
**created_at** | Option<**String**> | A custom date/time denoting _when_ the user signed up to the application, specified in RFC3339 format (e.g. `2012-10-20T07:15:20.902Z`). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


