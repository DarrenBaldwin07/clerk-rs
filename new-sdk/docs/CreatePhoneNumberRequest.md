# CreatePhoneNumberRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_id** | Option<**String**> | The ID representing the user | [optional]
**phone_number** | Option<**String**> | The new phone number. Must adhere to the E.164 standard for phone number format. | [optional]
**verified** | Option<**bool**> | When created, the phone number will be marked as verified. | [optional]
**primary** | Option<**bool**> | Create this phone number as the primary phone number for the user. Default: false, unless it is the first phone number. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


