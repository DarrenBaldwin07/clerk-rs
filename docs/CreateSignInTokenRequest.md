# CreateSignInTokenRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user that can use the newly created sign in token |
**org_id** | Option<**String**> | The ID of the organization to activate when the user signs in. Organizations must be enabled for the instance, and the user must be a member of the organization. | [optional]
**expires_in_seconds** | Option<**u32**> | Optional parameter to specify the life duration of the sign in token in seconds. By default, the duration is 30 days. | [optional][default to 2592000]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


