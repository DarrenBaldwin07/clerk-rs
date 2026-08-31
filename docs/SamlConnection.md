# SamlConnection

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** |  (enum: saml_connection) |
**id** | **String** |  |
**name** | **String** |  |
**domain** | Option<**String**> |  | [optional]
**domains** | Option<**Vec<String>**> |  | [optional]
**idp_entity_id** | Option<**String**> |  |
**idp_sso_url** | Option<**String**> |  |
**idp_certificate** | Option<**String**> |  |
**idp_certificate_issued_at** | Option<**i64**> | Unix timestamp (milliseconds) of the start of the IdP certificate validity window (X.509 NotBefore). Null when no certificate is configured.  |
**idp_certificate_expires_at** | Option<**i64**> | Unix timestamp (milliseconds) of the end of the IdP certificate validity window (X.509 NotAfter). Null when no certificate is configured.  |
**idp_metadata_url** | Option<**String**> |  | [optional]
**idp_metadata** | Option<**String**> |  | [optional]
**acs_url** | **String** |  |
**sp_entity_id** | **String** |  |
**sp_metadata_url** | **String** |  |
**organization_id** | Option<**String**> |  | [optional]
**attribute_mapping** | Option<[**models::SamlConnectionAttributeMapping**](SAMLConnectionAttributeMapping.md)> |  | [optional]
**active** | **bool** |  |
**provider** | **String** |  |
**user_count** | **i32** |  |
**sync_user_attributes** | **bool** |  |
**allow_subdomains** | **bool** |  |
**allow_idp_initiated** | **bool** |  |
**disable_additional_identifications** | **bool** |  |
**allow_organization_account_linking** | **bool** |  |
**force_authn** | **bool** | Enable or deactivate ForceAuthn |
**login_hint** | [**models::SamlConnectionLoginHint**](SAMLConnectionLoginHint.md) |  |
**enterprise_connection_id** | Option<**String**> |  | [optional]
**created_at** | **i64** | Unix timestamp of creation.  |
**updated_at** | **i64** | Unix timestamp of last update.  |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


