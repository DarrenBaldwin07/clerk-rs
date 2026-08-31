# UpdateSamlConnectionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the new SAML Connection | [optional]
**domain** | Option<**String**> | The domain to use for the new SAML Connection | [optional]
**domains** | Option<**Vec<String>**> | A list of the domains on use for the SAML connection | [optional]
**idp_entity_id** | Option<**String**> | The Entity ID as provided by the IdP | [optional]
**idp_sso_url** | Option<**String**> | The SSO URL as provided by the IdP | [optional]
**idp_certificate** | Option<**String**> | The x509 certificated as provided by the IdP | [optional]
**idp_metadata_url** | Option<**String**> | The URL which serves the IdP metadata. If present, it takes priority over the corresponding individual properties and replaces them | [optional]
**idp_metadata** | Option<**String**> | The XML content of the IdP metadata file. If present, it takes priority over the corresponding individual properties | [optional]
**organization_id** | Option<**String**> | The ID of the organization to which users of this SAML Connection will be added | [optional]
**attribute_mapping** | Option<[**models::CreateSamlConnectionRequestAttributeMapping**](CreateSAMLConnectionRequestAttributeMapping.md)> |  | [optional]
**active** | Option<**bool**> | Activate or de-activate the SAML Connection | [optional]
**sync_user_attributes** | Option<**bool**> | Controls whether to update the user's attributes in each sign-in | [optional]
**allow_subdomains** | Option<**bool**> | Allow users with an email address subdomain to use this connection in order to authenticate | [optional]
**allow_idp_initiated** | Option<**bool**> | Enable or deactivate IdP-initiated flows | [optional]
**disable_additional_identifications** | Option<**bool**> | Enable or deactivate additional identifications | [optional]
**allow_organization_account_linking** | Option<**bool**> | Whether this connection supports account linking via organization membership | [optional]
**force_authn** | Option<**bool**> | Enable or deactivate ForceAuthn | [optional]
**login_hint** | Option<[**models::CreateSamlConnectionRequestLoginHint**](CreateSAMLConnectionRequestLoginHint.md)> |  | [optional]
**consent_verified_domains_deletion** | Option<**bool**> | When enabling the connection, controls behavior when verified domains used for enrollment modes like automatic invitation or automatic suggestion already exist for the same domain. If true, those verified domains are removed and the connection is enabled. If false or omitted, the request fails when any such verified domain exists. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


