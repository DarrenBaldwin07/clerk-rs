# UpdateSignUpRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**custom_action** | Option<**bool**> | Specifies whether a custom action has run for this sign-up attempt. This is important when your instance has been configured to require a custom action to run before converting a sign-up into a user. After executing any external business logic you deem necessary, you can mark the sign-up as ready-to-convert by setting `custom_action` to `true`. | [optional]
**external_id** | Option<**String**> | The ID of the guest attempting to sign up as used in your external systems or your previous authentication solution. This will be copied to the resulting user when the sign-up is completed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


