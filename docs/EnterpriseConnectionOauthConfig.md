# EnterpriseConnectionOauthConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | OAuth config ID | [optional]
**name** | Option<**String**> | Custom OIDC provider display name | [optional]
**provider_key** | Option<**String**> | OAuth provider key (e.g. oidc_custom, oidc_ghe_*, oidc_gitlab_ent_*) | [optional]
**client_id** | Option<**String**> | OAuth client ID | [optional]
**discovery_url** | Option<**String**> | OIDC discovery URL | [optional]
**auth_url** | Option<**String**> | OAuth authorization endpoint URL (present when configured or resolved from discovery) | [optional]
**token_url** | Option<**String**> | OAuth token endpoint URL (present when configured or resolved from discovery) | [optional]
**user_info_url** | Option<**String**> | OIDC userinfo endpoint URL (present when configured or resolved from discovery) | [optional]
**requires_pkce** | Option<**bool**> | Whether PKCE is required for this OAuth client | [optional]
**logo_public_url** | Option<**String**> | Logo URL for the provider | [optional]
**created_at** | Option<**i64**> | Unix timestamp in milliseconds when the config was created | [optional]
**updated_at** | Option<**i64**> | Unix timestamp in milliseconds when the config was last updated | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


