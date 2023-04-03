# UpdateOrganizationMembershipMetadataRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**public_metadata** | Option<[**serde_json::Value**](.md)> | Metadata saved on the organization membership, that is visible to both your frontend and backend. The new object will be merged with the existing value. | [optional]
**private_metadata** | Option<[**serde_json::Value**](.md)> | Metadata saved on the organization membership that is only visible to your backend. The new object will be merged with the existing value. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


