# \SessionsApi

All URIs are relative to *https://api.clerk.dev/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_session_token_from_template**](SessionsApi.md#create_session_token_from_template) | **POST** /sessions/{session_id}/tokens/{template_name} | Create a session token from a jwt template
[**get_session**](SessionsApi.md#get_session) | **GET** /sessions/{session_id} | Retrieve a session
[**get_session_list**](SessionsApi.md#get_session_list) | **GET** /sessions | List all sessions
[**revoke_session**](SessionsApi.md#revoke_session) | **POST** /sessions/{session_id}/revoke | Revoke a session
[**verify_session**](SessionsApi.md#verify_session) | **POST** /sessions/{session_id}/verify | Verify a session



## create_session_token_from_template

> crate::models::CreateSessionTokenFromTemplate200Response create_session_token_from_template(session_id, template_name)
Create a session token from a jwt template

Creates a JSON Web Token(JWT) based on a session and a JWT Template name defined for your instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session | [required] |
**template_name** | **String** | The name of the JWT Template defined in your instance (e.g. `custom_hasura`). | [required] |

### Return type

[**crate::models::CreateSessionTokenFromTemplate200Response**](CreateSessionTokenFromTemplate_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_session

> crate::models::Session get_session(session_id)
Retrieve a session

Retrieve the details of a session

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session | [required] |

### Return type

[**crate::models::Session**](Session.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_session_list

> Vec<crate::models::Session> get_session_list(client_id, user_id, status, limit, offset)
List all sessions

Returns a list of all sessions. The sessions are returned sorted by creation date, with the newest sessions appearing first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | Option<**String**> | List sessions for the given client |  |
**user_id** | Option<**String**> | List sessions for the given user |  |
**status** | Option<**String**> | Filter sessions by the provided status |  |
**limit** | Option<**f32**> | Applies a limit to the number of results returned. Can be used for paginating the results together with `offset`. Must be an integer greater than zero and less than 500. By default, if not supplied, a limit of 10 is used. |  |[default to 10]
**offset** | Option<**f32**> | Skip the first `offset` results when paginating. Needs to be an integer greater or equal to zero. To be used in conjunction with `limit`. |  |[default to 0]

### Return type

[**Vec<crate::models::Session>**](Session.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_session

> crate::models::Session revoke_session(session_id)
Revoke a session

Sets the status of a session as \"revoked\", which is an unauthenticated state. In multi-session mode, a revoked session will still be returned along with its client object, however the user will need to sign in again.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session | [required] |

### Return type

[**crate::models::Session**](Session.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_session

> crate::models::Session verify_session(session_id, verify_session_request)
Verify a session

Returns the session if it is authenticated, otherwise returns an error.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session | [required] |
**verify_session_request** | Option<[**VerifySessionRequest**](VerifySessionRequest.md)> | Parameters. |  |

### Return type

[**crate::models::Session**](Session.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

