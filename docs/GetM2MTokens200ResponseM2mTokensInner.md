# GetM2MTokens200ResponseM2mTokensInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** |  (enum: machine_to_machine_token) |
**id** | **String** |  |
**subject** | **String** |  |
**claims** | Option<**serde_json::Value**> |  | [optional]
**scopes** | Option<**Vec<String>**> |  | [optional][default to []]
**revoked** | **bool** |  |
**revocation_reason** | Option<**String**> |  |
**expired** | **bool** |  |
**expiration** | Option<**f64**> | The timestamp for when the token will expire, in milliseconds |
**last_used_at** | Option<**f64**> | The timestamp for when the token was last used, in milliseconds |
**created_at** | **f64** | The timestamp for when the token was created, in milliseconds |
**updated_at** | **f64** | The timestamp for when the token was last updated, in milliseconds |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


