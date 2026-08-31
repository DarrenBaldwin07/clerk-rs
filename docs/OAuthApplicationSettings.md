# OAuthApplicationSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** | String representing the object's type. Objects of the same type share the same value. (enum: oauth_application_settings) |
**dynamic_oauth_client_registration** | **bool** | Whether dynamic OAuth client registration is enabled for the instance (RFC 7591). |
**default_scopes** | Option<**HashSet<String>**> | Default scopes. |
**oauth_jwt_access_tokens** | **bool** | Whether OAuth JWT access tokens are enabled for the instance (disabled indicates opaque access tokens). |
**client_id_metadata_documents_advertised** | **bool** | Whether the instance advertises support for Client ID Metadata Documents in its OAuth authorization server metadata. |
**client_id_metadata_documents_only_allow_pre_registered_clients** | **bool** | When true, new unknown CIMD clients are rejected. Previously auto-connected and pre-registered clients remain admitted; deleting a client makes it unknown again. |
**client_id_metadata_documents_block_implicitly_allowed_clients** | **bool** | When true, recorded implicitly allowed CIMD clients are rejected on future client lookups. Explicitly allowed clients remain accepted. This does not revoke previously issued access tokens. |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


