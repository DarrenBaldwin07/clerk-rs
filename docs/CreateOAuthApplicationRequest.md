# CreateOAuthApplicationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the new OAuth application. Max length: 256 |
**redirect_uris** | Option<**Vec<String>**> | An array of redirect URIs of the new OAuth application | [optional]
**callback_url** | Option<**String**> | The callback URL of the new OAuth application | [optional]
**scopes** | Option<**String**> | Define the allowed scopes for the new OAuth applications that dictate the user payload of the OAuth user info endpoint. Available scopes are `profile`, `email`, `public_metadata`, `private_metadata`. Provide the requested scopes as a string, separated by spaces. | [optional][default to profile email]
**consent_screen_enabled** | Option<**bool**> | True to enable a consent screen to display in the authentication flow. | [optional][default to true]
**pkce_required** | Option<**bool**> | True to require the Proof Key of Code Exchange (PKCE) flow. | [optional][default to false]
**public** | Option<**bool**> | If true, this client is public and you can use the Proof Key of Code Exchange (PKCE) flow. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


