# ReplaceUserMetadataRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**public_metadata** | Option<**std::collections::HashMap<String, serde_json::Value>**> | Metadata saved on the user, that is visible to both your frontend and backend. The existing value will be replaced entirely with the new object. | [optional]
**private_metadata** | Option<**std::collections::HashMap<String, serde_json::Value>**> | Metadata saved on the user that is only visible to your backend. The existing value will be replaced entirely with the new object. | [optional]
**unsafe_metadata** | Option<**std::collections::HashMap<String, serde_json::Value>**> | Metadata saved on the user, that can be updated from both the Frontend and Backend APIs. The existing value will be replaced entirely with the new object.  Note: Since this data can be modified from the frontend, it is not guaranteed to be safe. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


