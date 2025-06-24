# SmsMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **String** | The type of object, always "sms_message" | 
**id** | **String** | Unique identifier for the SMS message | 
**slug** | Option<**String**> | A URL-friendly identifier for the SMS message template | [optional]
**from_phone_number** | **String** | The phone number that sent the SMS message in E.164 format | 
**to_phone_number** | **String** | The phone number that received the SMS message in E.164 format | 
**phone_number_id** | Option<**String**> | The ID of the phone number associated with the SMS message | 
**user_id** | Option<**String**> | The ID of the user associated with the SMS message | [optional]
**message** | **String** | The content of the SMS message | 
**status** | **String** | The current status of the SMS message (e.g., "sent", "delivered", "failed") | 
**data** | Option<[**serde_json::Value**](.md)> | Additional metadata associated with the SMS message | [optional]
**delivered_by_clerk** | **bool** | Indicates whether the SMS was delivered through Clerk's messaging service | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


