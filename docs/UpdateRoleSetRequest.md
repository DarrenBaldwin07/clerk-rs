# UpdateRoleSetRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The new name for the role set | [optional]
**key** | Option<**String**> | A unique key for the role set. Must start with 'role_set:' and contain only lowercase alphanumeric characters and underscores. | [optional]
**description** | Option<**String**> | Optional description for the role set | [optional]
**r#type** | Option<**Type**> | Set to \"initial\" to make this the default role set for new organizations. Only one role set can be \"initial\" per instance; setting this will change any existing initial role set to \"custom\". (enum: initial) | [optional]
**default_role_key** | Option<**String**> | The key of the role to use as the default role for new organization members. Must be an existing role in the role set. | [optional]
**creator_role_key** | Option<**String**> | The key of the role to assign to organization creators. Must be an existing role in the role set. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


