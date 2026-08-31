# UpdateOrganizationRoleRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The new name for the organization role | [optional]
**key** | Option<**String**> | A unique key for the organization role. Must start with 'org:' and contain only lowercase alphanumeric characters and underscores. | [optional]
**description** | Option<**String**> | Optional description for the role | [optional]
**permissions** | Option<**Vec<String>**> | Array of permission IDs to assign to the role. If provided, this will replace the existing permissions. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


