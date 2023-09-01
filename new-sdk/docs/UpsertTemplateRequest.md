# UpsertTemplateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The user-friendly name of the template | [optional]
**subject** | Option<**String**> | The email subject. Applicable only to email templates. | [optional]
**markup** | Option<**String**> | The editor markup used to generate the body of the template | [optional]
**body** | Option<**String**> | The template body before variable interpolation | [optional]
**delivered_by_clerk** | Option<**bool**> | Whether Clerk should deliver emails or SMS messages based on the current template | [optional]
**from_email_name** | Option<**String**> | The local part of the From email address that will be used for emails. For example, in the address 'hello@example.com', the local part is 'hello'. Applicable only to email templates. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


