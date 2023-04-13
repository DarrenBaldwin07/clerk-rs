# UpdateInstanceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**test_mode** | Option<**bool**> | Toggles test mode for this instance, allowing the use of test email addresses and phone numbers. Defaults to true for development instances. | [optional]
**hibp** | Option<**bool**> | Whether the instance should be using the HIBP service to check passwords for breaches | [optional]
**enhanced_email_deliverability** | Option<**bool**> | The \"enhanced_email_deliverability\" feature will send emails from \"verifications@clerk.dev\" instead of your domain. This can be helpful if you do not have a high domain reputation. | [optional]
**support_email** | Option<**String**> |  | [optional]
**clerk_js_version** | Option<**String**> |  | [optional]
**experimental_allowed_origins** | Option<**Vec<String>**> |  | [optional]
**allowed_origins** | Option<**Vec<String>**> |  | [optional]
**cookieless_dev** | Option<**bool**> | Whether the instance should operate in cookieless development mode (i.e. without third-party cookies). Deprecated: Please use `url_based_session_syncing` instead. | [optional]
**url_based_session_syncing** | Option<**bool**> | Whether the instance should use URL-based session syncing in development mode (i.e. without third-party cookies). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


