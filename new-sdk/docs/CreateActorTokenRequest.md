# CreateActorTokenRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user that can use the newly created sign in token. | 
**actor** | [**serde_json::Value**](.md) | The actor payload. It needs to include a sub property which should contain the ID of the actor. This whole payload will be also included in the JWT session token. | 
**expires_in_seconds** | Option<**i32**> | Optional parameter to specify the life duration of the actor token in seconds. By default, the duration is 1 hour. | [optional][default to 3600]
**session_max_duration_in_seconds** | Option<**i32**> | The maximum duration that the session which will be created by the generated actor token should last. By default, the duration of a session created via an actor token, lasts 30 minutes. | [optional][default to 1800]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


