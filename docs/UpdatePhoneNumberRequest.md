# UpdatePhoneNumberRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**verified** | Option<**bool**> | The phone number will be marked as verified. | [optional]
**primary** | Option<**bool**> | Set this phone number as the primary phone number for the user. | [optional]
**reserved_for_second_factor** | Option<**bool**> | Set this phone number as reserved for multi-factor authentication. The phone number must also be verified. If there are no other reserved second factors, the phone number will be set as the default second factor. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


