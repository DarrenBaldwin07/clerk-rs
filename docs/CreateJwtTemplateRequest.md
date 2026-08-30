# CreateJwtTemplateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | JWT template name |
**claims** | **serde_json::Value** | JWT template claims in JSON format |
**lifetime** | Option<**u32**> | JWT lifetime | [optional]
**allowed_clock_skew** | Option<**u32**> | JWT allowed clock skew | [optional]
**custom_signing_key** | Option<**bool**> | Whether a custom signing key/algorithm is also provided for this template | [optional]
**signing_algorithm** | Option<**String**> | The custom signing algorithm to use when minting JWTs. Required if `custom_signing_key` is `true`. | [optional]
**signing_key** | Option<**String**> | The custom signing private key to use when minting JWTs. Required if `custom_signing_key` is `true`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


