# UpdateEnterpriseConnectionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The display name of the enterprise connection | [optional]
**domains** | Option<**Vec<String>**> | Domains associated with the enterprise connection. Values are normalized to lowercase. Empty array means ignored (no change); non-empty array means set domains to the given list (replaces existing). | [optional]
**active** | Option<**bool**> | Whether the enterprise connection is active. When set to true (enabling), any existing verified organization domains for the same domain(s) will be removed so the connection can be enabled. | [optional]
**sync_user_attributes** | Option<**bool**> | Whether to sync user attributes on sign-in | [optional]
**disable_additional_identifications** | Option<**bool**> | Whether to disable additional identifications | [optional]
**allow_organization_account_linking** | Option<**bool**> | Whether this connection supports account linking via organization membership | [optional]
**organization_id** | Option<**String**> | Organization ID to link to this enterprise connection. Only linking is supported; sending this field sets or changes the linked organization. There is no way to unlink an organization once linked. | [optional]
**saml** | Option<[**models::UpdateEnterpriseConnectionRequestSaml**](UpdateEnterpriseConnectionRequestSaml.md)> |  | [optional]
**oidc** | Option<[**models::UpdateEnterpriseConnectionRequestOidc**](UpdateEnterpriseConnectionRequestOidc.md)> |  | [optional]
**custom_attributes** | Option<[**Vec<models::CreateEnterpriseConnectionRequestCustomAttributesInner>**](CreateEnterpriseConnectionRequestCustomAttributesInner.md)> | Custom attributes to map from the IdP to the user's profile via SSO or SCIM provisioning. Requires the custom attributes feature to be enabled for the instance. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


