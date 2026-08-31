# VerificationOtp

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | Option<**Object**> |  (enum: verification_otp) | [optional]
**status** | **Status** |  (enum: unverified, verified, failed, expired) |
**strategy** | **Strategy** |  (enum: phone_code, email_code, reset_password_email_code) |
**attempts** | Option<**i32**> |  |
**expire_at** | Option<**i64**> |  |
**channel** | Option<**String**> | The delivery channel of the code (phone codes only). | [optional]
**verified_at_client** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


