# CreateMachine200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** |  (enum: machine) |
**id** | **String** | Unique identifier for the machine. |
**name** | **String** | The name of the machine. |
**instance_id** | **String** | The ID of the instance this machine belongs to. |
**created_at** | **i64** | Unix timestamp of creation. |
**updated_at** | **i64** | Unix timestamp of last update. |
**default_token_ttl** | Option<**u32**> | The default time-to-live (TTL) in seconds for tokens created by this machine. | [optional][default to 3600]
**scoped_machines** | [**Vec<models::MachineWithoutScopedMachines>**](MachineWithoutScopedMachines.md) | Array of machines this machine has access to. |
**secret_key** | **String** | The secret key for the machine, only returned upon creation. | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


