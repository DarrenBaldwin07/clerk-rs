# Organization

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** |  (enum: organization) |
**id** | **String** |  |
**name** | **String** |  |
**slug** | **String** |  |
**image_url** | Option<**String**> |  | [optional]
**has_image** | **bool** |  |
**members_count** | Option<**i32**> |  | [optional]
**missing_member_with_elevated_permissions** | Option<**bool**> |  | [optional]
**pending_invitations_count** | Option<**i32**> |  | [optional]
**max_allowed_memberships** | **i32** |  |
**admin_delete_enabled** | **bool** |  |
**public_metadata** | **std::collections::HashMap<String, serde_json::Value>** |  |
**private_metadata** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]
**created_by** | Option<**String**> |  | [optional]
**created_at** | **i64** | Unix timestamp of creation.  |
**updated_at** | **i64** | Unix timestamp of last update.  |
**last_active_at** | Option<**i64**> | Unix timestamp of last activity.  | [optional]
**role_set_key** | Option<**String**> | The key of the [role set](https://clerk.com/docs/guides/organizations/control-access/role-sets) assigned to this organization.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


