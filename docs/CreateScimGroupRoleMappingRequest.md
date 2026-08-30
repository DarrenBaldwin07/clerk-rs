# CreateScimGroupRoleMappingRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**scim_group_id** | **String** | The SCIM group ID from the identity provider. |
**role_id** | **String** | The ID of the organization role to assign to members of the SCIM group. |
**precedence** | Option<**u32**> | The precedence for this mapping. Lower values take priority when a user belongs to multiple mapped groups. If omitted, the mapping is appended with the next-highest precedence. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


