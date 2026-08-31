# CreateM2MTokenRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**token_format** | Option<**TokenFormat**> |  (enum: opaque, jwt) | [optional][default to Opaque]
**seconds_until_expiration** | Option<**f64**> |  | [optional]
**claims** | Option<**serde_json::Value**> |  | [optional]
**min_remaining_ttl_seconds** | Option<**u32**> | Enables server-side token reuse for opaque-format tokens. When set, if a non-revoked, non-expired M2M token already exists for this machine with identical `claims` and `scopes` and at least this many seconds of remaining lifetime, that existing token is returned and no new token is minted.  Use this when caching tokens in application memory across requests is impractical — for example, in serverless functions, short-lived job workers, or autoscaling containers that churn faster than the token TTL. Pooling at the server collapses many redundant create calls into reuse of a single live token, which is the recommended pattern for high-volume M2M traffic.  Must be strictly less than the effective token lifetime — that is, `seconds_until_expiration` when provided, or the machine's default TTL otherwise. A value greater than or equal to the lifetime is rejected with a 400, since no freshly-minted token would ever satisfy the requirement.  Only applies to opaque-format tokens (`token_format` defaults to `opaque`). JWT-format tokens are stateless and are never deduplicated. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


