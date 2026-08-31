# Invitation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** |  (enum: invitation) |
**id** | **String** |  |
**email_address** | **String** |  |
**public_metadata** | **std::collections::HashMap<String, serde_json::Value>** |  |
**revoked** | Option<**bool**> |  | [optional]
**status** | **Status** |  (enum: pending, accepted, revoked, expired) |
**url** | Option<**String**> |  | [optional]
**expires_at** | Option<**i64**> | Unix timestamp of expiration.  | [optional]
**created_at** | **i64** | Unix timestamp of creation.  |
**updated_at** | **i64** | Unix timestamp of last update.  |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


