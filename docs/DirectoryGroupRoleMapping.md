# DirectoryGroupRoleMapping

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** | String representing the object's type. Always \"directory_group_role_mapping\". (enum: directory_group_role_mapping) |
**id** | **String** | Unique identifier for the directory group role mapping. |
**directory_id** | **String** | The ID of the directory this mapping belongs to. |
**scim_directory_id** | **String** | The legacy name for `directory_id`. Carries the same value. |
**directory_group_id** | **String** | The group ID from the identity provider. |
**scim_group_id** | **String** | The legacy name for `directory_group_id`. Carries the same value. |
**directory_group_display_name** | **String** | The display name of the group, as reported by the identity provider. |
**scim_group_display_name** | **String** | The legacy name for `directory_group_display_name`. Carries the same value. |
**role** | Option<[**models::Role**](Role.md)> |  | [optional]
**precedence** | **i32** | Mapping precedence, starting at 1. Lower values take priority when a user belongs to multiple mapped groups. |
**created_at** | **i64** | Unix timestamp (milliseconds) of when the mapping was created. |
**updated_at** | **i64** | Unix timestamp (milliseconds) of when the mapping was last updated. |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


