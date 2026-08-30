# CreateMachineRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the machine |
**scoped_machines** | Option<**Vec<String>**> | Array of machine IDs that this machine will have access to. Maximum of 150 scopes per machine. | [optional]
**default_token_ttl** | Option<**u32**> | The default time-to-live (TTL) in seconds for tokens created by this machine. Must be at least 1 second. | [optional][default to 3600]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


