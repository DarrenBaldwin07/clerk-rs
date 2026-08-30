# UpdateInstanceOAuthApplicationSettingsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dynamic_oauth_client_registration** | Option<**bool**> | Whether dynamic OAuth client registration is enabled for the instance (RFC 7591). | [optional]
**default_scopes** | Option<**Vec<String>**> | Default scopes. Set to null to reset to Clerk-provided defaults. | [optional]
**oauth_jwt_access_tokens** | Option<**bool**> | Whether OAuth JWT access tokens are enabled for the instance (disabled indicates opaque access tokens). | [optional]
**client_id_metadata_documents_advertised** | Option<**bool**> | Whether the instance advertises support for Client ID Metadata Documents in its OAuth authorization server metadata. | [optional]
**client_id_metadata_documents_only_allow_pre_registered_clients** | Option<**bool**> | When true, new unknown CIMD clients are rejected. Previously auto-connected and pre-registered clients remain admitted; deleting a client makes it unknown again. | [optional]
**client_id_metadata_documents_block_implicitly_allowed_clients** | Option<**bool**> | When true, recorded implicitly allowed CIMD clients are rejected on future client lookups. Explicitly allowed clients remain accepted. This does not revoke previously issued access tokens. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


