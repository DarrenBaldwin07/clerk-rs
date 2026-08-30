# RotateMachineSecretKeyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**previous_token_ttl** | **u32** | The time in seconds that the previous secret key will remain valid after rotation. This ensures a graceful transition period for updating applications with the new secret key. Set to 0 to immediately expire the previous key. Maximum value is 8 hours (28800 seconds). |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


