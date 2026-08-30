# PrepareEmailAddressVerification200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | Option<**String**> | The type of the verification object. | [optional]
**id** | Option<**String**> | The ID of the verification. Pass this to attempt_verification. | [optional]
**status** | Option<**String**> | The status of the verification (unverified, verified, expired, or failed). | [optional]
**strategy** | Option<**String**> | The verification strategy (email_code or phone_code). | [optional]
**attempts** | Option<**i32**> | The number of attempts made against this verification. | [optional]
**expire_at** | Option<**i64**> | Unix timestamp (milliseconds) at which the code expires. | [optional]
**channel** | Option<**String**> | The channel the code was sent over (phone numbers only). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


