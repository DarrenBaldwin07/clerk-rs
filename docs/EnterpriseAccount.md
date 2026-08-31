# EnterpriseAccount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  |
**object** | **Object** | String representing the object's type. Objects of the same type share the same value.  (enum: enterprise_account) |
**protocol** | Option<**Protocol**> | The authentication protocol used to sign in.  (enum: oauth, saml) | [optional]
**provider** | **String** |  |
**active** | **bool** |  |
**email_address** | **String** |  |
**first_name** | Option<**String**> |  | [optional]
**last_name** | Option<**String**> |  | [optional]
**provider_user_id** | Option<**String**> | The unique ID of the user in the external provider's system | [optional]
**enterprise_connection_id** | Option<**String**> |  | [optional]
**public_metadata** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]
**verification** | Option<[**models::EnterpriseAccountVerification**](EnterpriseAccountVerification.md)> |  |
**enterprise_connection** | Option<[**models::EnterpriseAccountConnection**](EnterpriseAccountConnection.md)> |  | [optional]
**last_authenticated_at** | Option<**i64**> | Unix timestamp of last authentication.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


