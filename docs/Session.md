# Session

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** | String representing the object's type. Objects of the same type share the same value.  (enum: session) |
**id** | **String** |  |
**user_id** | **String** |  |
**client_id** | **String** |  |
**actor** | Option<**serde_json::Value**> |  | [optional]
**status** | **Status** |  (enum: active, revoked, ended, expired, removed, abandoned, replaced, pending) |
**last_active_organization_id** | Option<**String**> |  | [optional]
**last_active_at** | **i64** |  |
**latest_activity** | Option<[**models::SessionActivityResponse**](SessionActivityResponse.md)> |  | [optional]
**expire_at** | **i64** | Unix timestamp of expiration.  |
**abandon_at** | **i64** | Unix timestamp of abandonment.  |
**updated_at** | **i64** | Unix timestamp of last update.  |
**created_at** | **i64** | Unix timestamp of creation.  |
**tasks** | Option<[**Vec<models::SessionTask>**](SessionTask.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


