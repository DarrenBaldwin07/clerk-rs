# CreateAgentTaskRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**on_behalf_of** | [**models::CreateAgentTaskRequestOnBehalfOf**](CreateAgentTaskRequestOnBehalfOf.md) |  |
**permissions** | **Permissions** | The permissions granted to the agent task. Must be \"*\" (all permissions). (enum: *) |
**agent_name** | **String** | A name identifying the agent. Used to derive a stable agent_id per instance. Logged for audit purposes. |
**task_description** | **String** | A description of the task being performed. Logged for audit purposes. |
**redirect_url** | **String** | The URL the user is redirected to after the agent task is accepted. Must be a valid absolute URL with an `https` scheme in production instances. In development instances, `http` is also permitted. The URL's domain must belong to one of the instance's associated domains (primary or satellite); otherwise the redirect will be rejected when the task ticket is consumed. |
**session_max_duration_in_seconds** | Option<**i32**> | The maximum duration that the session which will be created by the generated agent task should last. By default, the duration of a session created via an agent task lasts 30 minutes. | [optional][default to 1800]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


