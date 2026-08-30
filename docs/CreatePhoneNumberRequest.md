# CreatePhoneNumberRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_id** | **String** | The ID representing the user |
**phone_number** | **String** | The new phone number. Must adhere to the E.164 standard for phone number format. |
**verified** | Option<**bool**> | When created, the phone number will be marked as verified. | [optional]
**primary** | Option<**bool**> | Create this phone number as the primary phone number for the user. Default: false, unless it is the first phone number. | [optional]
**reserved_for_second_factor** | Option<**bool**> | Create this phone number as reserved for multi-factor authentication. The phone number must also be verified. If there are no other reserved second factors, the phone number will be set as the default second factor. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


