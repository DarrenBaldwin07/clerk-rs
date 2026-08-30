# ScimDirectoryCustomAttributesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Display name for the custom attribute | [optional]
**key** | Option<**String**> | Key used to store the attribute in the user's public/private/unsafe metadata | [optional]
**sso_path** | Option<**String**> | Path to extract the attribute value from SSO claims (SAML assertions or OIDC claims) | [optional]
**scim_path** | Option<**String**> | GJSON path to extract the attribute value from SCIM user resources | [optional]
**directory_path** | Option<**String**> | GJSON path to extract the attribute value from directory sync user resources. Same value as `scim_path`. | [optional]
**multi_valued** | Option<**bool**> | When true, the attribute supports multiple values; values from the IdP are written to public_metadata as an array. Defaults to false. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


