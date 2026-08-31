# OAuthApplicationWithSecret

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** |  (enum: oauth_application) |
**id** | **String** |  |
**instance_id** | **String** |  |
**name** | **String** |  |
**client_id** | **String** |  |
**client_uri** | Option<**String**> |  |
**client_image_url** | Option<**String**> |  |
**dynamically_registered** | **bool** |  |
**consent_screen_enabled** | **bool** |  |
**pkce_required** | **bool** |  |
**public** | **bool** |  |
**scopes** | **String** |  |
**redirect_uris** | **Vec<String>** |  |
**callback_url** | **String** | Deprecated: Use redirect_uris instead.  |
**authorize_url** | **String** |  |
**token_fetch_url** | **String** |  |
**user_info_url** | **String** |  |
**discovery_url** | **String** |  |
**token_introspection_url** | **String** |  |
**created_at** | **i64** | Unix timestamp of creation.  |
**updated_at** | **i64** | Unix timestamp of last update.  |
**client_secret** | Option<**String**> | Empty if public client.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


