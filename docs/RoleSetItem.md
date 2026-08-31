# RoleSetItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** |  (enum: role_set_item) |
**id** | **String** | The unique identifier of the role |
**name** | **String** | The name of the role |
**key** | **String** | The key of the role (e.g., \"org:admin\", \"org:member\") |
**description** | Option<**String**> | Optional description of the role |
**members_count** | Option<**i64**> | The number of members assigned to this role within the role set | [optional]
**has_members** | Option<**bool**> | Whether this role has any members assigned within the role set | [optional]
**created_at** | **i64** | Unix timestamp of role creation |
**updated_at** | **i64** | Unix timestamp of last role update |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


