# User

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  |
**object** | **Object** | String representing the object's type. Objects of the same type share the same value.  (enum: user) |
**external_id** | Option<**String**> |  |
**primary_email_address_id** | Option<**String**> |  |
**primary_phone_number_id** | Option<**String**> |  |
**primary_web3_wallet_id** | Option<**String**> |  |
**username** | Option<**String**> |  |
**first_name** | Option<**String**> |  |
**last_name** | Option<**String**> |  |
**locale** | Option<**String**> |  | [optional]
**profile_image_url** | Option<**String**> |  | [optional]
**image_url** | Option<**String**> |  | [optional]
**has_image** | **bool** |  |
**public_metadata** | **std::collections::HashMap<String, serde_json::Value>** |  |
**private_metadata** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]
**unsafe_metadata** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]
**email_addresses** | [**Vec<models::EmailAddress>**](EmailAddress.md) |  |
**phone_numbers** | [**Vec<models::PhoneNumber>**](PhoneNumber.md) |  |
**web3_wallets** | [**Vec<models::Web3Wallet>**](Web3Wallet.md) |  |
**passkeys** | [**Vec<models::Passkey>**](Passkey.md) |  |
**password_enabled** | **bool** |  |
**two_factor_enabled** | **bool** |  |
**totp_enabled** | **bool** |  |
**backup_code_enabled** | **bool** |  |
**mfa_enabled_at** | Option<**i64**> | Unix timestamp of when MFA was last enabled for this user. It should be noted that this field is not nullified if MFA is disabled.  |
**mfa_disabled_at** | Option<**i64**> | Unix timestamp of when MFA was last disabled for this user. It should be noted that this field is not nullified if MFA is enabled again.  |
**password_last_updated_at** | Option<**i64**> | Unix timestamp of when the user's password was last updated.  | [optional]
**external_accounts** | [**Vec<models::ExternalAccountWithVerification>**](ExternalAccountWithVerification.md) |  |
**saml_accounts** | [**Vec<models::SamlAccount>**](SAMLAccount.md) |  |
**enterprise_accounts** | [**Vec<models::EnterpriseAccount>**](EnterpriseAccount.md) |  |
**organization_memberships** | Option<[**Vec<models::OrganizationMembership>**](OrganizationMembership.md)> |  | [optional]
**last_sign_in_at** | Option<**i64**> | Unix timestamp of last sign-in.  |
**banned** | **bool** | Flag to denote whether user is banned or not.  |
**locked** | **bool** | Flag to denote whether user is currently locked, i.e. restricted from signing in or not.  |
**deprovisioned** | Option<**bool**> | Flag to denote whether user has been deprovisioned and is restricted from signing in.  | [optional]
**lockout_expires_in_seconds** | Option<**i64**> | The number of seconds remaining until the lockout period expires for a locked user. A null value for a locked user indicates that lockout never expires.  |
**verification_attempts_remaining** | Option<**i64**> | The number of verification attempts remaining until the user is locked. Null if account lockout is not enabled. Note: if a user is locked explicitly via the Backend API, they may still have verification attempts remaining.  |
**updated_at** | **i64** | Unix timestamp of last update.  |
**created_at** | **i64** | Unix timestamp of creation.  |
**delete_self_enabled** | **bool** | If enabled, user can delete themselves via FAPI.  |
**create_organization_enabled** | **bool** | If enabled, user can create organizations via FAPI.  |
**create_organizations_limit** | Option<**i32**> | The maximum number of organizations the user can create. 0 means unlimited.  | [optional]
**last_active_at** | Option<**i64**> | Unix timestamp of the latest session activity, with day precision.  |
**legal_accepted_at** | Option<**i64**> | Unix timestamp of when the user accepted the legal requirements.  |
**bypass_client_trust** | Option<**bool**> | When set to `true`, the user will bypass Device Trust checks during sign-in. | [optional][default to false]
**scim** | Option<[**models::ScimUserMetadata**](SCIMUserMetadata.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


