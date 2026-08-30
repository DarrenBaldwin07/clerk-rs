# ScimGroupRoleMapping

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** | String representing the object's type. Always \"scim_group_role_mapping\". (enum: scim_group_role_mapping) |
**id** | **String** | Unique identifier for the SCIM group role mapping. |
**scim_directory_id** | **String** | The ID of the directory this mapping belongs to. |
**directory_id** | **String** | The ID of the directory this mapping belongs to. Same value as `scim_directory_id`. |
**scim_group_id** | **String** | The SCIM group ID from the identity provider. |
**directory_group_id** | **String** | The group ID from the identity provider. Same value as `scim_group_id`. |
**scim_group_display_name** | **String** | The display name of the SCIM group, as reported by the identity provider. |
**directory_group_display_name** | **String** | The display name of the group, as reported by the identity provider. Same value as `scim_group_display_name`. |
**role** | Option<[**models::Role**](Role.md)> |  | [optional]
**precedence** | **i32** | Mapping precedence, starting at 1. Lower values take priority when a user belongs to multiple mapped groups. |
**created_at** | **i64** | Unix timestamp (milliseconds) of when the mapping was created. |
**updated_at** | **i64** | Unix timestamp (milliseconds) of when the mapping was last updated. |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


