# \AgentTasksApi

All URIs are relative to *https://api.clerk.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_agent_task**](AgentTasksApi.md#create_agent_task) | **POST** /agents/tasks | Create agent task
[**revoke_agent_task**](AgentTasksApi.md#revoke_agent_task) | **POST** /agents/tasks/{agent_task_id}/revoke | Revoke agent task



## create_agent_task

> models::AgentTask create_agent_task(create_agent_task_request)
Create agent task

Create an agent task on behalf of a user. The response contains a URL that, when visited, creates a session for the user. The agent_id is stable per agent_name within an instance. The agent_task_id is unique per call.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_agent_task_request** | Option<[**CreateAgentTaskRequest**](CreateAgentTaskRequest.md)> |  |  |

### Return type

[**models::AgentTask**](AgentTask.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_agent_task

> models::AgentTask revoke_agent_task(agent_task_id)
Revoke agent task

Revokes a pending agent task.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_task_id** | **String** | The ID of the agent task to be revoked. | [required] |

### Return type

[**models::AgentTask**](AgentTask.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

