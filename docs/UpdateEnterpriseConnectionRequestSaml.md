# UpdateEnterpriseConnectionRequestSaml

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Display name of the SAML connection | [optional]
**idp_entity_id** | Option<**String**> | IdP entity ID | [optional]
**idp_sso_url** | Option<**String**> | IdP SSO URL | [optional]
**idp_certificate** | Option<**String**> | IdP certificate (PEM) | [optional]
**idp_metadata_url** | Option<**String**> | URL to IdP metadata | [optional]
**idp_metadata** | Option<**String**> | Raw IdP metadata XML | [optional]
**attribute_mapping** | Option<[**models::CreateEnterpriseConnectionRequestSamlAttributeMapping**](CreateEnterpriseConnectionRequestSamlAttributeMapping.md)> |  | [optional]
**allow_subdomains** | Option<**bool**> |  | [optional]
**allow_idp_initiated** | Option<**bool**> |  | [optional]
**force_authn** | Option<**bool**> |  | [optional]
**login_hint** | Option<[**models::CreateSamlConnectionRequestLoginHint**](CreateSAMLConnectionRequestLoginHint.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


