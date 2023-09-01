# CreateJwtTemplateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | JWT template name | [optional]
**claims** | Option<[**serde_json::Value**](.md)> | JWT template claims in JSON format | [optional]
**lifetime** | Option<**f32**> | JWT token lifetime | [optional]
**allowed_clock_skew** | Option<**f32**> | JWT token allowed clock skew | [optional]
**custom_signing_key** | Option<**bool**> | Whether a custom signing key/algorithm is also provided for this template | [optional]
**signing_algorithm** | Option<**String**> | The custom signing algorithm to use when minting JWTs | [optional]
**signing_key** | Option<**String**> | The custom signing private key to use when minting JWTs | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


