# User

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**object** | Option<**String**> | String representing the object's type. Objects of the same type share the same value.  | [optional]
**external_id** | Option<**String**> |  | [optional]
**primary_email_address_id** | Option<**String**> |  | [optional]
**primary_phone_number_id** | Option<**String**> |  | [optional]
**primary_web3_wallet_id** | Option<**String**> |  | [optional]
**username** | Option<**String**> |  | [optional]
**first_name** | Option<**String**> |  | [optional]
**last_name** | Option<**String**> |  | [optional]
**profile_image_url** | Option<**String**> |  | [optional]
**public_metadata** | Option<[**serde_json::Value**](.md)> |  | [optional]
**private_metadata** | Option<[**serde_json::Value**](.md)> |  | [optional]
**unsafe_metadata** | Option<[**serde_json::Value**](.md)> |  | [optional]
**gender** | Option<**String**> |  | [optional]
**birthday** | Option<**String**> |  | [optional]
**email_addresses** | Option<[**Vec<crate::models::EmailAddress>**](EmailAddress.md)> |  | [optional]
**phone_numbers** | Option<[**Vec<crate::models::PhoneNumber>**](PhoneNumber.md)> |  | [optional]
**web3_wallets** | Option<[**Vec<crate::models::Web3Wallet>**](Web3Wallet.md)> |  | [optional]
**password_enabled** | Option<**bool**> |  | [optional]
**two_factor_enabled** | Option<**bool**> |  | [optional]
**totp_enabled** | Option<**bool**> |  | [optional]
**backup_code_enabled** | Option<**bool**> |  | [optional]
**external_accounts** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**last_sign_in_at** | Option<**i64**> | Unix timestamp of last sign-in.  | [optional]
**banned** | Option<**bool**> | Flag to denote whether user is banned or not.  | [optional]
**updated_at** | Option<**i64**> | Unix timestamp of last update.  | [optional]
**created_at** | Option<**i64**> | Unix timestamp of creation.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


