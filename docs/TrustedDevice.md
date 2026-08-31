# TrustedDevice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** | String representing the object's type. (enum: trusted_device) |
**id** | **String** |  |
**platform** | **Platform** |  (enum: ios, android) |
**app_identifier** | **String** |  |
**name** | Option<**String**> |  | [optional]
**algorithm** | **Algorithm** |  (enum: ES256) |
**status** | **Status** |  (enum: active, revoked) |
**created_at** | **i64** | Unix timestamp of creation in milliseconds. |
**updated_at** | **i64** | Unix timestamp of the last update in milliseconds. |
**last_used_at** | Option<**i64**> | Unix timestamp of the last use in milliseconds. | [optional]
**revoked_at** | Option<**i64**> | Unix timestamp of revocation in milliseconds. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


