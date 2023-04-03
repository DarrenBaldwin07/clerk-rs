# UpdateUserMetadataRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**public_metadata** | Option<[**serde_json::Value**](.md)> | Metadata saved on the user, that is visible to both your frontend and backend. The new object will be merged with the existing value. | [optional]
**private_metadata** | Option<[**serde_json::Value**](.md)> | Metadata saved on the user that is only visible to your backend. The new object will be merged with the existing value. | [optional]
**unsafe_metadata** | Option<[**serde_json::Value**](.md)> | Metadata saved on the user, that can be updated from both the Frontend and Backend APIs. The new object will be merged with the existing value.  Note: Since this data can be modified from the frontend, it is not guaranteed to be safe. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


