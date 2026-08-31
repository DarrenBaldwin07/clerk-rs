# CreateEnterpriseConnectionRequestCustomAttributesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Display name for the custom attribute |
**key** | **String** | Key used to store the attribute in the user's metadata |
**sso_path** | Option<**String**> | Path to extract the attribute value from SSO claims | [optional]
**scim_path** | Option<**String**> | GJSON path to extract the attribute value from SCIM user resources | [optional]
**directory_path** | Option<**String**> | The new name for `scim_path`. Send either one, or both with the same value; sending both with different values is rejected. | [optional]
**multi_valued** | Option<**bool**> | When true, the attribute supports multiple values; values from the IdP are written to public_metadata as an array. Defaults to false. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


