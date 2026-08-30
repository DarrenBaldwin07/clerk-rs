# RoleSetMigration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** |  (enum: role_set_migration) |
**id** | **String** | The unique identifier of the migration |
**organization_id** | Option<**String**> | The organization ID if the migration is scoped to a specific organization | [optional]
**instance_id** | **String** | The instance ID this migration belongs to |
**source_role_set_id** | **String** | The ID of the source role set being migrated from |
**dest_role_set_id** | Option<**String**> | The ID of the destination role set being migrated to | [optional]
**trigger_type** | **String** | What triggered this migration (e.g., \"role_set_deletion\", \"role_removal\") |
**status** | **String** | Current status of the migration (e.g., \"enqueued\", \"in_progress\", \"completed\") |
**migrated_members** | **i32** | Number of members that have been migrated so far |
**mappings** | Option<**std::collections::HashMap<String, String>**> | Role key mappings from source to destination roles | [optional]
**started_at** | Option<**i64**> | Unix timestamp when the migration started | [optional]
**completed_at** | Option<**i64**> | Unix timestamp when the migration completed | [optional]
**created_at** | **i64** | Unix timestamp of migration creation |
**updated_at** | **i64** | Unix timestamp of last migration update |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


