# CreateDirectoryGroupRoleMappingRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**directory_group_id** | Option<**String**> | The group ID from the identity provider. Exactly one of `directory_group_id` or `scim_group_id` is required. | [optional]
**scim_group_id** | Option<**String**> | The legacy name for `directory_group_id`. Send either one, or both with the same value; sending both with different values is rejected. | [optional]
**role_id** | **String** | The ID of the organization role to assign to members of the group. |
**precedence** | Option<**u32**> | The precedence for this mapping. Lower values take priority when a user belongs to multiple mapped groups. If omitted, the mapping is appended with the next-highest precedence. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


