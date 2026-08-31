# \SessionsApi

All URIs are relative to *https://api.clerk.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_session**](SessionsApi.md#create_session) | **POST** /sessions | Create a new active session
[**create_session_token**](SessionsApi.md#create_session_token) | **POST** /sessions/{session_id}/tokens | Create a session token
[**create_session_token_from_template**](SessionsApi.md#create_session_token_from_template) | **POST** /sessions/{session_id}/tokens/{template_name} | Create a session token from a JWT template
[**get_session**](SessionsApi.md#get_session) | **GET** /sessions/{session_id} | Retrieve a session
[**get_session_list**](SessionsApi.md#get_session_list) | **GET** /sessions | List all sessions
[**refresh_session**](SessionsApi.md#refresh_session) | **POST** /sessions/{session_id}/refresh | Refresh a session
[**revoke_session**](SessionsApi.md#revoke_session) | **POST** /sessions/{session_id}/revoke | Revoke a session



## create_session

> models::Session create_session(create_session_request)
Create a new active session

Create a new active session for the provided user ID.  **This operation is intended only for use in testing, and is not available for production instances.** If you are looking to generate a user session from the backend, we recommend using the [Sign-in Tokens](https://clerk.com/docs/reference/backend-api/tag/Sign-in-Tokens#operation/CreateSignInToken) resource instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_session_request** | Option<[**CreateSessionRequest**](CreateSessionRequest.md)> |  |  |

### Return type

[**models::Session**](Session.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_session_token

> models::CreateSessionToken200Response create_session_token(session_id, create_session_token_request)
Create a session token

Creates a session JSON Web Token (JWT) based on a session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session | [required] |
**create_session_token_request** | Option<[**CreateSessionTokenRequest**](CreateSessionTokenRequest.md)> |  |  |

### Return type

[**models::CreateSessionToken200Response**](CreateSessionToken_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_session_token_from_template

> models::CreateSessionToken200Response create_session_token_from_template(session_id, template_name, create_session_token_from_template_request)
Create a session token from a JWT template

Creates a JSON Web Token (JWT) based on a session and a JWT Template name defined for your instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session | [required] |
**template_name** | **String** | The name of the JWT template defined in your instance (e.g. `custom_hasura`). | [required] |
**create_session_token_from_template_request** | Option<[**CreateSessionTokenFromTemplateRequest**](CreateSessionTokenFromTemplateRequest.md)> |  |  |

### Return type

[**models::CreateSessionToken200Response**](CreateSessionToken_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_session

> models::Session get_session(session_id)
Retrieve a session

Retrieve the details of a session

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session | [required] |

### Return type

[**models::Session**](Session.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_session_list

> Vec<models::Session> get_session_list(client_id, user_id, status, paginated, limit, offset)
List all sessions

Returns a list of sessions matching the provided criteria. The sessions are returned sorted by creation date, with the newest sessions appearing first.  Note: This endpoint does not return all sessions that have ever existed. Old and inactive sessions are periodically cleaned up and will not be included in the results.  **Deprecation Notice (2024-01-01):** All parameters were initially considered optional, however moving forward at least one of `client_id` or `user_id` parameters should be provided.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | Option<**String**> | List sessions for the given client |  |
**user_id** | Option<**String**> | List sessions for the given user |  |
**status** | Option<**String**> | Filter sessions by the provided status |  |
**paginated** | Option<**bool**> | Whether to paginate the results. If true, the results will be paginated. If false, the results will not be paginated. |  |
**limit** | Option<**u32**> | Applies a limit to the number of results returned. Can be used for paginating the results together with `offset`. |  |[default to 10]
**offset** | Option<**u32**> | Skip the first `offset` results when paginating. Needs to be an integer greater or equal to zero. To be used in conjunction with `limit`. |  |[default to 0]

### Return type

[**Vec<models::Session>**](Session.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_session

> models::SessionRefresh refresh_session(session_id, refresh_session_request)
Refresh a session

Refreshes a session by creating a new session token. A 401 is returned when there are validation errors, which signals the SDKs to fall back to the handshake flow.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session | [required] |
**refresh_session_request** | Option<[**RefreshSessionRequest**](RefreshSessionRequest.md)> | Refresh session parameters |  |

### Return type

[**models::SessionRefresh**](SessionRefresh.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_session

> models::Session revoke_session(session_id)
Revoke a session

Sets the status of a session as \"revoked\", which is an unauthenticated state. In multi-session mode, a revoked session will still be returned along with its client object, however the user will need to sign in again.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The ID of the session | [required] |

### Return type

[**models::Session**](Session.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

