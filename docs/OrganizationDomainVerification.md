# OrganizationDomainVerification

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | **String** | Status of the verification. It can be `unverified`, `verified`, `failed`, or `expired`. |
**strategy** | **String** | Name of the strategy used to verify the domain |
**attempts** | Option<**i32**> | How many attempts have been made to verify the domain |
**expire_at** | Option<**i64**> | Unix timestamp of when the verification will expire |
**verified_at** | Option<**i64**> | Unix timestamp of when ownership was verified. Only populated on `ownership_verification`; null on `affiliation_verification`.  |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


