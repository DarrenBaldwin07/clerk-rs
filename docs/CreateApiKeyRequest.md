# CreateApiKeyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> |  | [optional][default to api_key]
**name** | **String** |  |
**description** | Option<**String**> |  | [optional]
**subject** | **String** |  |
**claims** | Option<**serde_json::Value**> |  | [optional]
**scopes** | Option<**Vec<String>**> |  | [optional][default to []]
**created_by** | Option<**String**> |  | [optional]
**seconds_until_expiration** | Option<**f64**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


