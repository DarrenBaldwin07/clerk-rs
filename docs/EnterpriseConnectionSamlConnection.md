# EnterpriseConnectionSamlConnection

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | SAML connection ID | [optional]
**name** | Option<**String**> | SAML connection display name | [optional]
**idp_entity_id** | Option<**String**> | IdP entity ID (optional, when connection details are loaded) | [optional]
**idp_sso_url** | Option<**String**> | IdP SSO URL (optional, when connection details are loaded) | [optional]
**idp_metadata_url** | Option<**String**> | IdP metadata URL (optional, when connection details are loaded) | [optional]
**acs_url** | Option<**String**> | Assertion Consumer Service URL | [optional]
**sp_entity_id** | Option<**String**> | Service Provider entity ID | [optional]
**sp_metadata_url** | Option<**String**> | Service Provider metadata URL | [optional]
**active** | Option<**bool**> | Whether the SAML connection is active | [optional]
**allow_idp_initiated** | Option<**bool**> | Whether IdP-initiated SSO is allowed | [optional]
**allow_subdomains** | Option<**bool**> | Whether subdomains are allowed for domain matching | [optional]
**force_authn** | Option<**bool**> | Whether to force re-authentication | [optional]
**login_hint** | Option<[**models::EnterpriseConnectionSamlConnectionLoginHint**](EnterpriseConnectionSamlConnectionLoginHint.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


