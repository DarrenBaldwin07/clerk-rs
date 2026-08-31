# UpdateInstanceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**test_mode** | Option<**bool**> | Toggles test mode for this instance, allowing the use of test email addresses and phone numbers. Defaults to true for development instances. | [optional]
**hibp** | Option<**bool**> | Whether the instance should be using the HIBP service to check passwords for breaches | [optional]
**support_email** | Option<**String**> |  | [optional]
**clerk_js_version** | Option<**String**> |  | [optional]
**development_origin** | Option<**String**> |  | [optional]
**allowed_origins** | Option<**Vec<String>**> | For browser-like stacks such as browser extensions, Electron (not officially supported), or Capacitor.js (not officially supported), the instance allowed origins need to be updated with the request origin value. For Chrome extensions popup, background, or service worker pages, the origin is chrome-extension://extension_uuid. For Electron apps the default origin is http://localhost:3000. For Capacitor, the origin is capacitor://localhost. | [optional]
**cookieless_dev** | Option<**bool**> | Whether the instance should operate in cookieless development mode (i.e. without third-party cookies). Deprecated: Please use `url_based_session_syncing` instead. | [optional]
**url_based_session_syncing** | Option<**bool**> | Whether the instance should use URL-based session syncing in development mode (i.e. without third-party cookies). | [optional]
**preferred_sign_in_strategy_when_password_required** | Option<**PreferredSignInStrategyWhenPasswordRequired**> | When password is required at the instance level, sets the preferred sign-in strategy surfaced to Clerk components. Has no effect when password is not required. Defaults to `password`. Set to an empty string to clear the override. (enum: password, otp, ) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


