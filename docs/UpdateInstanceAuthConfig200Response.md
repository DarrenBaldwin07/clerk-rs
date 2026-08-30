# UpdateInstanceAuthConfig200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | Option<**Object**> | String representing the object's type. Objects of the same type share the same value. (enum: instance_settings) | [optional]
**id** | Option<**String**> |  | [optional]
**restricted_to_allowlist** | Option<**bool**> |  | [optional]
**from_email_address** | Option<**String**> |  | [optional]
**progressive_sign_up** | Option<**bool**> |  | [optional]
**enhanced_email_deliverability** | Option<**bool**> | Deprecated. When enabled, production authentication emails for this instance are sent through Clerk's legacy managed email delivery path. This setting is being retired; use the instance's configured email sending domain instead.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


