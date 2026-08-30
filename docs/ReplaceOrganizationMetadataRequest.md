# ReplaceOrganizationMetadataRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**public_metadata** | Option<**std::collections::HashMap<String, serde_json::Value>**> | Metadata saved on the organization, that is visible to both your frontend and backend. The existing value will be replaced entirely with the new object. | [optional]
**private_metadata** | Option<**std::collections::HashMap<String, serde_json::Value>**> | Metadata saved on the organization that is only visible to your backend. The existing value will be replaced entirely with the new object. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


