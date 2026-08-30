# CreateOrganizationMembershipRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user that will be added as a member in the organization. The user needs to exist in the same instance as the organization and must not be a member of the given organization already. |
**role** | **String** | The role that the new member will have in the organization. |
**public_metadata** | Option<**std::collections::HashMap<String, serde_json::Value>**> | Metadata saved on the organization membership, that is visible to both your frontend and backend. | [optional]
**private_metadata** | Option<**std::collections::HashMap<String, serde_json::Value>**> | Metadata saved on the organization membership that is only visible to your backend. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


