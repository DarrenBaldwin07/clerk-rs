# GetApiKeys200ResponseDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** |  (enum: api_key) |
**id** | **String** |  |
**r#type** | **String** |  |
**subject** | **String** |  |
**name** | **String** |  |
**description** | Option<**String**> |  | [optional]
**claims** | Option<**serde_json::Value**> |  |
**scopes** | **Vec<String>** |  |
**revoked** | **bool** |  |
**revocation_reason** | Option<**String**> |  |
**expired** | **bool** |  |
**expiration** | Option<**f64**> | The timestamp for when the API key will expire, in milliseconds |
**created_by** | Option<**String**> |  |
**last_used_at** | Option<**f64**> | The timestamp for when the API key was last used, in milliseconds |
**created_at** | **f64** | The timestamp for when the API key was created, in milliseconds |
**updated_at** | **f64** | The timestamp for when the API key was last updated, in milliseconds |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


