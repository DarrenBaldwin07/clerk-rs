# EnterpriseConnection

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** |  (enum: enterprise_connection) |
**id** | **String** | The enterprise connection ID |
**name** | **String** | The display name of the connection |
**provider** | **String** |  |
**logo_public_url** | Option<**String**> |  | [optional]
**active** | **bool** | Whether the enterprise connection is active |
**domains** | **Vec<String>** | Domains associated with the enterprise connection |
**organization_id** | Option<**String**> | Organization ID when the connection is linked to an organization | [optional]
**sync_user_attributes** | Option<**bool**> | Controls whether to update the user's attributes on each sign-in | [optional]
**disable_additional_identifications** | Option<**bool**> | When true, users cannot add additional identifications when using this connection | [optional]
**allow_organization_account_linking** | Option<**bool**> | Whether this connection supports account linking via organization membership | [optional]
**custom_attributes** | Option<[**Vec<models::EnterpriseConnectionCustomAttributesInner>**](EnterpriseConnectionCustomAttributesInner.md)> | Custom attributes to map from the IdP to the user's profile via SSO or SCIM provisioning | [optional]
**saml_connection** | Option<[**models::EnterpriseConnectionSamlConnection**](EnterpriseConnectionSamlConnection.md)> |  | [optional]
**oauth_config** | Option<[**models::EnterpriseConnectionOauthConfig**](EnterpriseConnectionOauthConfig.md)> |  | [optional]
**created_at** | **i64** | Unix timestamp in milliseconds when the connection was created |
**updated_at** | **i64** | Unix timestamp in milliseconds when the connection was last updated |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


