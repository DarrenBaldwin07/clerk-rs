# GetOAuthAccessToken200ResponseInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | Option<**String**> |  | [optional]
**token** | Option<**String**> | The access token | [optional]
**provider** | Option<**String**> | The ID of the provider | [optional]
**public_metadata** | Option<[**serde_json::Value**](.md)> |  | [optional]
**label** | Option<**String**> |  | [optional]
**scopes** | Option<**Vec<String>**> | The list of scopes that the token is valid for. Only present for OAuth 2.0 tokens. | [optional]
**token_secret** | Option<**String**> | The token secret. Only present for OAuth 1.0 tokens. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


