# CreateSamlConnectionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name to use as a label for this SAML Connection |
**domain** | Option<**String**> | The domain of your organization. Sign in flows using an email with this domain, will use this SAML Connection. | [optional]
**domains** | Option<**Vec<String>**> | The domains of your organization. Sign in flows using an email with one of these domains, will use this SAML Connection. | [optional]
**provider** | **Provider** | The IdP provider of the connection. (enum: saml_custom, saml_okta, saml_google, saml_microsoft) |
**idp_entity_id** | Option<**String**> | The Entity ID as provided by the IdP | [optional]
**idp_sso_url** | Option<**String**> | The Single-Sign On URL as provided by the IdP | [optional]
**idp_certificate** | Option<**String**> | The X.509 certificate as provided by the IdP | [optional]
**idp_metadata_url** | Option<**String**> | The URL which serves the IdP metadata. If present, it takes priority over the corresponding individual properties | [optional]
**idp_metadata** | Option<**String**> | The XML content of the IdP metadata file. If present, it takes priority over the corresponding individual properties | [optional]
**organization_id** | Option<**String**> | The ID of the organization to which users of this SAML Connection will be added | [optional]
**attribute_mapping** | Option<[**models::CreateSamlConnectionRequestAttributeMapping**](CreateSAMLConnectionRequestAttributeMapping.md)> |  | [optional]
**force_authn** | Option<**bool**> | Enable or deactivate ForceAuthn | [optional]
**login_hint** | Option<[**models::CreateSamlConnectionRequestLoginHint**](CreateSAMLConnectionRequestLoginHint.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


