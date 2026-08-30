# EnterpriseConnectionTestRun

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The test run ID |
**status** | **Status** | The status of the test run (enum: pending, success, failed) |
**connection_type** | **ConnectionType** | The type of enterprise connection the test run was performed against (enum: saml, oauth) |
**parsed_user_info** | Option<[**models::EnterpriseConnectionTestRunParsedUserInfo**](EnterpriseConnectionTestRunParsedUserInfo.md)> |  | [optional]
**logs** | Option<[**Vec<models::EnterpriseConnectionTestRunLogsInner>**](EnterpriseConnectionTestRunLogsInner.md)> | Log entries captured during the test run | [optional]
**saml** | Option<[**models::EnterpriseConnectionTestRunSaml**](EnterpriseConnectionTestRunSaml.md)> |  | [optional]
**oauth** | Option<[**models::EnterpriseConnectionTestRunOauth**](EnterpriseConnectionTestRunOauth.md)> |  | [optional]
**created_at** | **i64** | Unix timestamp in milliseconds when the test run was created |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


