# CreateOrganizationRoleRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the new organization role |
**key** | **String** | A unique key for the organization role. Must start with 'org:' and contain only lowercase alphanumeric characters and underscores. |
**description** | Option<**String**> | Optional description for the role | [optional]
**permissions** | Option<**Vec<String>**> | Array of permission IDs to assign to the role | [optional]
**include_in_initial_role_set** | Option<**bool**> | Whether this role should be included in the initial role set | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


