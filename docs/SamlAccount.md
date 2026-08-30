# SamlAccount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  |
**object** | **Object** | String representing the object's type. Objects of the same type share the same value.  (enum: saml_account) |
**provider** | **String** |  |
**active** | **bool** |  |
**email_address** | **String** |  |
**first_name** | Option<**String**> |  | [optional]
**last_name** | Option<**String**> |  | [optional]
**provider_user_id** | Option<**String**> |  | [optional]
**last_authenticated_at** | Option<**i64**> | Unix timestamp of last authentication.  | [optional]
**public_metadata** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]
**verification** | Option<[**models::SamlAccountVerification**](SAMLAccountVerification.md)> |  |
**saml_connection** | Option<[**models::SamlAccountConnection**](SAMLAccountConnection.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


