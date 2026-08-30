# RoleSet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** |  (enum: role_set) |
**id** | **String** | The unique identifier of the role set |
**name** | **String** | The name of the role set |
**key** | **String** | A unique key for the role set (e.g., \"role_set:default\") |
**description** | Option<**String**> | Optional description of the role set |
**roles** | [**Vec<models::RoleSetItem>**](RoleSetItem.md) | The list of roles in this role set |
**default_role** | Option<[**models::RoleSetItem**](RoleSetItem.md)> | The default role assigned to new organization members | [optional]
**creator_role** | Option<[**models::RoleSetItem**](RoleSetItem.md)> | The role assigned to the creator of an organization | [optional]
**r#type** | **Type** | The type of the role set (\"initial\" or \"custom\") (enum: initial, custom) |
**role_set_migration** | Option<[**models::RoleSetMigration**](RoleSetMigration.md)> | Active migration information, only present when status is \"enqueued\" or \"in_progress\" | [optional]
**created_at** | **i64** | Unix timestamp of role set creation |
**updated_at** | **i64** | Unix timestamp of last role set update |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


