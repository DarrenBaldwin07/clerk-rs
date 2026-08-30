# VerificationSaml

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | Option<**Object**> |  (enum: verification_saml) | [optional]
**status** | **Status** |  (enum: unverified, verified, failed, expired, transferable) |
**strategy** | **Strategy** |  (enum: saml) |
**external_verification_redirect_url** | Option<**String**> |  | [optional]
**error** | Option<[**models::ClerkError**](ClerkError.md)> |  | [optional]
**expire_at** | Option<**i32**> |  | [optional]
**attempts** | Option<**i32**> |  |
**verified_at_client** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


