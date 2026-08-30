# CreateEnterpriseConnectionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The display name of the connection |
**provider** | **Provider** | The identity provider (e.g. saml_custom, oidc_custom, oidc_github_enterprise, oidc_gitlab) (enum: saml_custom, saml_okta, saml_google, saml_microsoft, oidc_custom, oidc_github_enterprise, oidc_gitlab) |
**domains** | **Vec<String>** | Domains associated with the enterprise connection (required; at least one). Values are normalized to lowercase. Each domain must be a valid fully qualified domain name. |
**organization_id** | Option<**String**> | Organization ID when the connection is linked to an organization | [optional]
**allow_organization_account_linking** | Option<**bool**> | Whether this connection supports account linking via organization membership | [optional]
**active** | Option<**bool**> | Whether the enterprise connection is active. When true, IdP metadata must be provided via the `saml` object. | [optional]
**saml** | Option<[**models::CreateEnterpriseConnectionRequestSaml**](CreateEnterpriseConnectionRequestSaml.md)> |  | [optional]
**oidc** | Option<[**models::CreateEnterpriseConnectionRequestOidc**](CreateEnterpriseConnectionRequestOidc.md)> |  | [optional]
**custom_attributes** | Option<[**Vec<models::CreateEnterpriseConnectionRequestCustomAttributesInner>**](CreateEnterpriseConnectionRequestCustomAttributesInner.md)> | Custom attributes to map from the IdP to the user's profile via SSO or SCIM provisioning. Requires the custom attributes feature to be enabled for the instance. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


