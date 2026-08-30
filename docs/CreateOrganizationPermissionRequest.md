# CreateOrganizationPermissionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the permission. |
**key** | **String** | The key of the permission. Must have the format \"org:feature:action\" where feature and action are segments consisting of lowercase letters, digits, or underscores, for example \"org:billing:manage\" or \"org:team:read\". Cannot begin with \"org:sys_\" as that prefix is reserved for system permissions. |
**description** | Option<**String**> | A description of the permission. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


